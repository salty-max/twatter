/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.{rs,html}"],
  theme: {
    extend: {
      colors: {
        primary: {
          DEFAULT: "#6c5ce7",
          light: "#a29bfe",
        },
        success: {
          DEFAULT: "#00b894",
          light: "#55efc4",
        },
        danger: {
          DEFAULT: "#d63031",
          light: "#ff7675",
        },
        warning: {
          DEFAULT: "#fdcb6e",
          light: "#ffeaa7",
        },
        light: "#efefef",
        hint: "#dfe6e9",
        dark: "#2d3436",
        grey: "#636e72",
        border: "#b2bec3",
      },
      fontFamily: {
        Poppins: ["Poppins, sans-serif"],
      },
      container: {
        center: true,
        padding: "1rem",
        screens: {
          lg: "1280px",
          xl: "1920px",
          "2xl": "2080px",
          "3xl": "3840px",
        },
      },
    },
  },
  plugins: [],
};
