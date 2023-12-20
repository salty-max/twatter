/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.{rs,html}"],
  theme: {
    extend: {
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
