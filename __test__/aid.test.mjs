import {describe, expect, it} from "vitest";
import {aidRegExp, genAid, parseAid} from "../index.js";

describe("aid", () => {
    it("should generate a valid aid", () => {
        const date = new Date();
        const gotAid = genAid(date);
        expect(gotAid).toMatch(RegExp(aidRegExp));
        expect(parseAid(gotAid).date.getTime()).toBe(date.getTime());
    });
});
