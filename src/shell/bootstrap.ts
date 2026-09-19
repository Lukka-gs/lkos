import { createEventBus } from "../core/event-bus";
import { desktop } from "../platform/desktop";
import { renderNotch } from "../ui/notch";

export async function bootstrap(root: HTMLElement) {
  const ui = renderNotch(root);
  const showError = (error: unknown) => {
    ui.status.textContent = `Erro: ${String(error)}`;
    ui.status.title = String(error);
  };
  const bus = createEventBus(showError);
  const unsubscribe = bus.subscribe("foundation.probe", (event) => {
    ui.status.textContent = `Evento nativo recebido · ${String(event.payload)}`;
  });
  ui.quit.onclick = () => {
    desktop.quit().catch(showError);
  };
  try {
    const unlisten = await desktop.connect(bus);
    window.addEventListener(
      "pagehide",
      () => {
        unlisten();
        unsubscribe();
      },
      { once: true },
    );
    const settings = await desktop.settings();
    ui.status.textContent = `Pronto · ${settings.probeCount} testes salvos`;
    ui.probe.disabled = false;
    ui.probe.onclick = async () => {
      ui.probe.disabled = true;
      try {
        await desktop.probe();
      } catch (error) {
        showError(error);
      } finally {
        ui.probe.disabled = false;
      }
    };
  } catch (error) {
    showError(error);
  }
}
