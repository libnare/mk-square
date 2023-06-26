import {describe, expect, it} from "vitest";
import {L_CHARS, secureRndstr} from "../index.js";

describe("secure_rndstr", () => {
    it("should generate a 32-character string consisting of numbers, uppercase letters, and lowercase letters if length is not specified", () => {
        const str = secureRndstr();
        expect(str.length).toBe(32);
        expect(str).toMatch(/^[0-9a-zA-Z]+$/);
    });
    it("should generate a 32-character string consisting of numbers and lowercase letters if length is not specified and L_CHARS is specified", () => {
        const str = secureRndstr(null, L_CHARS);
        expect(str.length).toBe(32);
        expect(str).toMatch(/^[0-9a-z]+$/);
    });

    it("should generate a 64-character string consisting of numbers, uppercase letters, and lowercase letters if length is specified as 64", () => {
        const str = secureRndstr(64);
        expect(str.length).toBe(64);
        expect(str).toMatch(/^[0-9a-zA-Z]+$/);
    });

    it("should generate a 64-character string consisting of numbers and lowercase letters if length is specified as 64 and L_CHARS is specified", () => {
        const str = secureRndstr(64, L_CHARS);
        expect(str.length).toBe(64);
        expect(str).toMatch(/^[0-9a-z]+$/);
    });
});