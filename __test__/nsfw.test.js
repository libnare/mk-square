const { detectSensitivity } = require('../index.js');

describe("nsfw", () => {
    it("should return null", async () => {
        const nsfw = await detectSensitivity(
            `${__dirname}/../assets/unknown`,
            "image/png",
            0.5,
            0.75,
            false,
        );
        expect(nsfw).toBeNull();
    }, 60000);
    it("should detect webp nsfw", async () => {
        let sensitive;
        let porn;
        const nsfw = await detectSensitivity(
            `${__dirname}/../assets/Clock_1.webp`,
            "image/png",
            0.5,
            0.75,
            false,
        );
        [sensitive, porn] = [nsfw.sensitive, nsfw.porn]
        expect(sensitive).toBe(false);
        expect(porn).toBe(false);
    }, 300000);
    it("should detect png nsfw", async () => {
        let sensitive;
        let porn;
        const nsfw = await detectSensitivity(
            `${__dirname}/../assets/Clock_1.png`,
            "image/png",
            0.5,
            0.75,
            false,
        );
        [sensitive, porn] = [nsfw.sensitive, nsfw.porn]
        expect(sensitive).toBe(false);
        expect(porn).toBe(false);
    }, 300000);
    it("should detect jpg nsfw", async () => {
        let sensitive;
        let porn;
        const nsfw = await detectSensitivity(
            `${__dirname}/../assets/Clock_1.jpg`,
            "image/png",
            0.5,
            0.75,
            false,
        );
        [sensitive, porn] = [nsfw.sensitive, nsfw.porn]
        expect(sensitive).toBe(false);
        expect(porn).toBe(false);
    }, 300000);
    it("should detect gif nsfw", async () => {
        const nsfw = await detectSensitivity(
            `${__dirname}/../assets/Clock_1.gif`,
            "image/gif",
            0.5,
            0.75,
            false,
        );
        expect(nsfw.sensitive).toBe(false);
        expect(nsfw.porn).toBe(false);
    }, 900000);
});