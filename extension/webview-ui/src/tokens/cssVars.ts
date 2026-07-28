/**
 * Maps canonical design token keys to CSS custom properties.
 * Source of truth: docs/ui/DESIGN_TOKENS.json
 */

export const designTokenCssVars: Record<string, string> = {
  "--oc-space-0": "0px",
  "--oc-space-1": "4px",
  "--oc-space-2": "8px",
  "--oc-space-3": "12px",
  "--oc-space-4": "16px",
  "--oc-space-5": "24px",
  "--oc-space-6": "32px",
  "--oc-space-7": "48px",
  "--oc-space-8": "64px",
  "--oc-radius-sm": "4px",
  "--oc-radius": "8px",
  "--oc-radius-lg": "12px",
  "--oc-radius-pill": "999px",
  "--oc-motion-fast": "150ms",
  "--oc-motion-normal": "200ms",
  "--oc-motion-slow": "250ms",
  "--oc-font-display-size": "28px",
  "--oc-font-display-weight": "700",
  "--oc-font-title-size": "20px",
  "--oc-font-title-weight": "650",
  "--oc-font-heading-size": "16px",
  "--oc-font-heading-weight": "650",
  "--oc-font-body-size": "13px",
  "--oc-font-body-weight": "400",
  "--oc-font-caption-size": "12px",
  "--oc-font-caption-weight": "400",
  "--oc-font-code-size": "13px",
  "--oc-font-code-weight": "400",
  "--oc-brand-night": "#0f172a",
  "--oc-brand-indigo": "#4338ca",
  "--oc-brand-eye": "#f59e0b",
  "--oc-brand-feather": "#e2e8f0",
};

/** Inject token CSS variables onto :root (optional runtime bootstrap). */
export function applyDesignTokenCssVars(root: HTMLElement = document.documentElement): void {
  for (const [key, value] of Object.entries(designTokenCssVars)) {
    root.style.setProperty(key, value);
  }
}

export function designTokenStyleBlock(): string {
  const lines = Object.entries(designTokenCssVars).map(
    ([key, value]) => `  ${key}: ${value};`
  );
  return `:root {\n${lines.join("\n")}\n}`;
}
