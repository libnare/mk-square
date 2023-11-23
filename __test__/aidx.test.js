const {aidxRegExp, genAidx, parseAidx} = require("../index.js");

describe("aid", () => {
    it("should generate a valid aid", () => {
        const date = Date.now();
        const gotAidx = genAidx(date);
        console.log(`AIDX: ${date}, ${gotAidx}`);
        expect(gotAidx).toMatch(RegExp(aidxRegExp));
        expect(parseAidx(gotAidx).date.getTime()).toBe(date);
    });
});
