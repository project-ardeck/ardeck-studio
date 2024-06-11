/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {},
    colors: { // color.adobe.com
      "white": "#FFFFFF",
      "black": "#000000",
      "bg-1": "#1D232B", // シェード ベースカラー
      "bg-2": "#3C4959",
      "bg-3": "#576A82",
      "bg-4": "#728BAB",
      "bg-4": "#8EACD4",
      "red": "#FF5050", // 正方形 ベースカラー
      "blue": "#4F5EFF",
      "green": "#4FFF81",
      "yellow": "#FFD94F"
    },
    fontFamily: {
      "0xp": ["0xProto Regular"]
    }
  },
  plugins: [],
}

