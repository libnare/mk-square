import {describe, expect, it} from "vitest";
import {parse, toString} from '../index.js';

describe("acct", () => {
    it('should parse acct correctly', () => {
        const acct1 = parse('@test@example.com');
        expect(acct1.username).toBe('test');
        expect(acct1.host).toBe('example.com');

        const acct3 = parse('test@example.com');
        expect(acct3.username).toBe('test');
        expect(acct3.host).toBe('example.com');

        const acct2 = parse('@test');
        expect(acct2.username).toBe('test');
        expect(acct2.host).toBeNull();
    });

    it('should convert acct object to string', () => {
        const acct1 = {username: 'test', host: 'example.com'};
        const str1 = toString(acct1);
        expect(str1).toBe('test@example.com');

        const acct2 = {username: 'test', host: null};
        const str2 = toString(acct2);
        expect(str2).toBe('test');
    });
});