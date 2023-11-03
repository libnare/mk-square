const {checkHttps} = require("../index.js");

describe("check_https", () => {
    it("should return true for an HTTPS URL", () => {
        expect(checkHttps("https://example.com", undefined)).toBe(true);
    });
    it("should return true for HTTP URL in development environment and false in production environment", () => {
        const url = "http://example.com";
        expect(checkHttps(url, "development")).toBe(true);
        expect(checkHttps(url, "production")).toBe(false);
    });
});
