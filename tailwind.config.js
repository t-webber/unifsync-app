/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html", 
    "./src/**.{vue, ts, js}"
  ],
  theme: {
    extend: {
      colors: {
        bg: 'var(--bg)', 
        fg: 'var(--fg)', 
        primary: 'var(--primary)', 
        muted: 'var(--muted)'
      }, 
    },
  },
  plugins: [],
}

