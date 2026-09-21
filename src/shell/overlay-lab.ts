import { lab, type LabAction } from "../platform/overlay-lab";
import "../ui/overlay-lab.css";

export async function startLab(root: HTMLElement) {
  document.body.classList.add("overlay-lab");
  root.innerHTML = `<h1>LUK-18 · Overlay Lab</h1>
<button id="target" type="button">Alvo sob o notch</button>
<p id="target-count" role="status">Cliques no alvo: 0</p>
<div class="lab-controls">
<button data-action="place-target">Posicionar sobre alvo</button>
<button data-action="passive">Ativar click-through</button>
<button data-action="restore">Restaurar interação</button>
<button data-action="menu">Menu nativo do tray</button>
<button data-action="default-position">Posição padrão</button>
<button id="refresh">Atualizar diagnóstico</button>
</div><p>Somente desenvolvimento. O alvo é uma janela distinta do mesmo processo; não substitui testes em outros apps.</p>
<pre id="snapshot" aria-label="Diagnóstico nativo"></pre>
<pre id="events" aria-label="Eventos de foco e menu"></pre>`;
  const target = root.querySelector<HTMLButtonElement>("#target");
  const count = root.querySelector<HTMLElement>("#target-count");
  const snapshot = root.querySelector<HTMLElement>("#snapshot");
  const events = root.querySelector<HTMLElement>("#events");
  const refresh = root.querySelector<HTMLButtonElement>("#refresh");
  if (!target || !count || !snapshot || !events || !refresh)
    throw Error("Missing lab controls");
  let clicks = 0;
  const update = async () => {
    try {
      snapshot.textContent = JSON.stringify(await lab.snapshot(), null, 2);
    } catch (error) {
      snapshot.textContent = String(error);
    }
  };
  target.onclick = () => {
    clicks++;
    count.textContent = `Cliques no alvo: ${clicks}`;
    void update();
  };
  refresh.onclick = () => {
    void update();
  };
  for (const button of root.querySelectorAll<HTMLButtonElement>(
    "[data-action]",
  )) {
    button.onclick = async () => {
      button.disabled = true;
      try {
        await lab.action(button.dataset.action as LabAction);
      } catch (error) {
        events.textContent = String(error);
      } finally {
        button.disabled = false;
        await update();
      }
    };
  }
  const dispose = await lab.observe((message) => {
    events.textContent = `${message}\n${events.textContent ?? ""}`.slice(
      0,
      2000,
    );
    void update();
  });
  window.addEventListener("pagehide", dispose, { once: true });
  await update();
}
