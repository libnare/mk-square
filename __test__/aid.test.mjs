import {describe, expect, it} from "vitest";
import {aidRegExp, genAid, parseAid} from "../index.js";

describe("aid", () => {
    it("should generate a valid aid", () => {
        const date = new Date();
        const gotAid = genAid(date);
        expect(gotAid).toMatch(RegExp(aidRegExp));
        expect(parseAid(gotAid).date.getTime()).toBe(date.getTime());
    });

    it("should generate a unique aid for each invocation", () => {
        const date1 = new Date();
        const date2 = new Date();
        const aid1 = genAid(date1);
        const aid2 = genAid(date2);
        expect(aid1).not.toBe(aid2);
    });
});
