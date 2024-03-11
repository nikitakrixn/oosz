const { fontFamily } = require("tailwindcss/defaultTheme");

/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "jit",
  content: {
    files: ["../templates/**/*.html", "../src/**/*.rs"],
  },
  theme: {
    extend: {
      boxShadow: {
        '0x1': '0 0 0 3px rgba(0, 0, 0, 0.3)',
      },
      fontFamily: {
        sans: ["Inter", ...fontFamily.sans],
      },
    },
  },
  darkMode: "class",
};
