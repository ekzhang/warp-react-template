const { createProxyMiddleware } = require("http-proxy-middleware");

module.exports = function (app) {
  app.use(
    "/graphql",
    createProxyMiddleware({
      target: "http://localhost:3535",
      changeOrigin: true,
    })
  );
};
