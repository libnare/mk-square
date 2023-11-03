const { correctFilename } = require("../index.js");

describe("corrent_filename", () => {
    it('returns the filename as is if it already ends with the correct extension', () => {
        const filename = 'image.png';
        const ext = 'png';
        expect(correctFilename(filename, ext)).toBe(filename);
    });

    it('returns the filename as is if the extension is "jpg" and the filename ends with ".jpeg"', () => {
        const filename = 'photo.jpeg';
        const ext = 'jpg';
        expect(correctFilename(filename, ext)).toBe(filename);
    });

    it('returns the filename as is if the extension is "tif" and the filename ends with ".tiff"', () => {
        const filename = 'scan.tiff';
        const ext = 'tif';
        expect(correctFilename(filename, ext)).toBe(filename);
    });

    it('appends the correct extension to the filename if it does not already have it', () => {
        const filename = 'document';
        const ext = 'pdf';
        expect(correctFilename(filename, ext)).toBe('document.pdf');
    });

    it('appends ".unknown" extension to the filename if ext is null', () => {
        const filename = 'file';
        const ext = null;
        expect(correctFilename(filename, ext)).toBe('file.unknown');
    });

    it('appends ".unknown" extension to the filename if ext is not provided', () => {
        const filename = 'file';
        expect(correctFilename(filename)).toBe('file.unknown');
    });
});
