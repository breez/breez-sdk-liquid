import * as Crypto from "expo-crypto";

import { english } from "./english";

export type EntropyStrength = 128 | 160 | 192 | 224 | 256;
export type MnemonicLanguage = "english"

/**
 * NOTE: This cannot be tested with jest as expo-crypto runs native code.
 *
 * It does work correctly though on the ios-simulator and development profiles
 * as per the manual tests, ie the mnemonic displayed by the app is correct.
 *
 * We need to come up with an E2E strategy for all cases described in the test
 * vectors of reference.
 *
 * [Maestro](https://docs.expo.dev/build-reference/e2e-tests/) could be the
 * solution.
 */
export class Mnemonic {
  static wordlists = {
    english,
  } satisfies Record<MnemonicLanguage, Array<string>>;

  private readonly wordlist: Array<string>;
  private readonly delimiter: string;

  constructor(
    language: MnemonicLanguage = "english",
    wordlist?: Array<string>,
  ) {
    if (wordlist !== undefined && wordlist.length !== 2048) {
      throw new TypeError("Provided wordlist is not 2,048 words");
    }

    this.wordlist = wordlist ?? Mnemonic.wordlists[language];
    this.delimiter = " ";
  }

  private static generateEntropy(strength: EntropyStrength = 128) {
    return Crypto.getRandomBytes(strength / 8);
  }

  private static toBinaryString(bytes: Uint8Array) {
    return bytes.reduce(
      (str, byte) => str + byte.toString(2).padStart(8, "0"),
      "",
    );
  }

  private static toByte(binary: string) {
    return parseInt(binary, 2);
  }

  private static async deriveChecksumBits(entropy: Uint8Array) {
    const ent = entropy.length * 8;
    const cs = ent / 32;
    const hash = await Crypto.digest(
      Crypto.CryptoDigestAlgorithm.SHA256,
      entropy,
    );

    return Mnemonic.toBinaryString(new Uint8Array(hash)).slice(0, cs);
  }

  async generateMnemonic(strength: EntropyStrength = 128) {
    const entropy = Mnemonic.generateEntropy(strength);

    return this.toMnemonic(entropy);
  }

  async toMnemonic(entropy: Uint8Array) {
    if (![16, 20, 24, 28, 32].includes(entropy.length)) {
      throw new TypeError(
        `Entropy length must be one of the following: [16, 20, 24, 28, 32], but it is ${entropy.length}`,
      );
    }

    const entropyBits = Mnemonic.toBinaryString(entropy);
    const checkSumBits = await Mnemonic.deriveChecksumBits(entropy);

    const bits = entropyBits + checkSumBits;
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const chunks = bits.match(/.{1,11}/g)!;
    const words = chunks.map(
      (binary) => this.wordlist[Mnemonic.toByte(binary)],
    );

    return words.join(this.delimiter);
  }
}
