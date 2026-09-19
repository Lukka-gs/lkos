import type { Gadget, ModuleContext, Tool } from "../shared/contracts";

type Module = Gadget | Tool;
/** Composition root supplies factories. Core never imports concrete modules. */
export class ModuleRegistry {
  private readonly modules = new Map<string, Module>();
  private readonly active = new Map<string, AbortController>();

  register(module: Module): void {
    if (this.modules.has(module.manifest.id))
      throw new Error(`Duplicate module: ${module.manifest.id}`);
    this.modules.set(module.manifest.id, module);
  }

  async start(
    id: string,
    context: Omit<ModuleContext, "signal">,
  ): Promise<void> {
    const module = this.modules.get(id);
    if (!module) throw new Error(`Unknown module: ${id}`);
    if (this.active.has(id)) throw new Error(`Module already active: ${id}`);
    if (module.manifest.permissions.some((p) => !context.permissions.has(p)))
      throw new Error(`Missing permission: ${id}`);
    const controller = new AbortController();
    this.active.set(id, controller);
    try {
      await module.start({ ...context, signal: controller.signal });
    } catch (error) {
      controller.abort();
      this.active.delete(id);
      try {
        await module.dispose();
      } catch {
        /* Preserve the startup error. */
      }
      throw error;
    }
  }

  async stop(id: string): Promise<void> {
    const controller = this.active.get(id);
    if (!controller) return;
    controller.abort();
    this.active.delete(id);
    await this.modules.get(id)?.dispose();
  }
}
