/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {},
    colors: { // color.adobe.com
      "bg-titlebar": "rgb(var(--bg-titlebar) / <alpha-value>)",
      "bg-primary": "rgb(var(--bg-primary) / <alpha-value>)",
      "bg-secondary": "rgb(var(--bg-secondary) / <alpha-value>)",
      "bg-tertiary": "rgb(var(--bg-tertiary) / <alpha-value>)",
      "bg-quaternary": "rgb(var(--bg-quaternary) / <alpha-value>)",
      "text-primary": "rgb(var(--text-primary) / <alpha-value>)",
      "text-secondary": "rgb(var(--text-secondary) / <alpha-value>)",
      "text-tertiary": "rgb(var(--text-tertiary) / <alpha-value>)",
      "text-reverse": "rgb(var(--text-reverse) / <alpha-value>)",
      "accent-primary": "rgb(var(--accent-primary) / <alpha-value>)",
      "accent-secondary": "rgb(var(--accent-secondary) / <alpha-value>)",
      "accent-positive": "rgb(var(--accent-positive) / <alpha-value>)",
      "accent-caution": "rgb(var(--accent-caution) / <alpha-value>)",
      "accent-negative": "rgb(var(--accent-negative) / <alpha-value>)",
      "accent-link": "rgb(var(--accent-link) / <alpha-value>)",
      // Using modern `rgb`
    },
    fontFamily: {
      "0xp": ["'0xProto Regular'"],
      "fordev": ["'monaspace'", "'notosans'"],
    }
  },
  plugins: [],
}

