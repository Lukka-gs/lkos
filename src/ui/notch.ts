export function renderNotch(root: HTMLElement) {
  root.innerHTML = `<section class="notch"><span class="mark" aria-hidden="true">L</span><div class="copy"><strong>LKOS <small>FOUNDATION</small></strong><p role="status" id="status">Conectando…</p></div><button id="probe" type="button" aria-label="Testar persistência e eventos" disabled>↗</button><button id="quit" type="button" aria-label="Encerrar LKOS">×</button></section>`;
  const status = root.querySelector<HTMLParagraphElement>("#status");
  const probe = root.querySelector<HTMLButtonElement>("#probe");
  const quit = root.querySelector<HTMLButtonElement>("#quit");
  if (!status || !probe || !quit) throw new Error("Incomplete notch markup");
  return { status, probe, quit };
}
