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
        let date = new Date();
        const generatedAids = new Set();
        const duplicatedAids = new Set();

        for (let i = 0; i < 12960; i++) {
            const aid = genAid(date);
            expect(aid).toMatch(RegExp(aidRegExp));

            if (generatedAids.has(aid)) {
                duplicatedAids.add(aid);
            } else {
                generatedAids.add(aid);
            }

            if (i % 1296 === 0) {
                date.setSeconds(date.getSeconds() + 1);
            }
        }

        expect(duplicatedAids.size).toBe(0);
    });
});
