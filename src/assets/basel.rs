pub const CSS: &str = r###":root:has(input.theme-controller[value=rust-ember]:checked),
[data-theme="rust-ember"] {
  color-scheme: dark;

  --color-base-100: oklch(22% 0.02 30); /* dark charcoal background */
  --color-base-200: oklch(28% 0.025 35);
  --color-base-300: oklch(35% 0.03 40);
  --color-base-content: oklch(88% 0.03 20); /* light text on dark */

  --color-primary: oklch(70% 0.22 35); /* warm glowing orange */
  --color-primary-content: oklch(15% 0.02 25); /* dark on orange */

  --color-secondary: oklch(72% 0.25 10); /* deep hot pink */
  --color-secondary-content: oklch(15% 0.02 25); /* dark on pink */

  --color-accent: oklch(68% 0.18 50); /* coral/pink-orange blend */
  --color-accent-content: oklch(15% 0.02 25);

  --color-neutral: oklch(28% 0.01 40); /* slightly blue-ish dark gray */
  --color-neutral-content: oklch(85% 0.02 25);

  --color-info: oklch(65% 0.12 280); /* violet-ish info pop */
  --color-info-content: oklch(15% 0.02 25);

  --color-success: oklch(65% 0.15 150); /* soft green */
  --color-success-content: oklch(15% 0.02 25);

  --color-warning: oklch(72% 0.24 70); /* warm yellow-orange */
  --color-warning-content: oklch(15% 0.02 25);

  --color-error: oklch(70% 0.25 25); /* warm red-orange */
  --color-error-content: oklch(15% 0.02 25);

  --radius-selector: 0.25rem;
  --radius-field: 0.25rem;
  --radius-box: 0.5rem;
  --size-selector: 0.25rem;
  --size-field: 0.25rem;
  --border: 1px;
  --depth: 0;
  --noise: 0;
}
"###;
