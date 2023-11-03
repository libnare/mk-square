const {aidRegExp, genAid, parseAid} = require("../index.js");

describe("aid", () => {
    it("should generate a valid aid", () => {
        const date = Date.now();
        const gotAid = genAid(date);
        expect(gotAid).toMatch(RegExp(aidRegExp));
        expect(parseAid(gotAid).date.getTime()).toBe(date);
    });
});
