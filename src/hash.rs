export class hash {
  _block: Buffer;
  _finalSize: number;
  _blockSize: number;
  _len: number;

  constructor(blockSize: number, finalSize: number) {
    this._block = Buffer.alloc(blockSize);
    this._finalSize = finalSize;
    this._blockSize = blockSize;
    this._len = 0;
  }

  update(data: string | Buffer, enc?: BufferEncoding) {
    // console.log("[hash] update", data, enc);
    "string" == typeof data &&
      ((enc = enc || "utf8"), (data = Buffer.from(data, enc)));
    for (
      var block = this._block,
        blockSize = this._blockSize,
        length = data.length,
        accum = this._len,
        offset = 0;
      offset < length;

    ) {
      for (
        var assigned = accum % blockSize,
          remainder = Math.min(length - offset, blockSize - assigned),
          i = 0;
        i < remainder;
        i++
      )
        block[assigned + i] = data[offset + i];
      (offset += remainder),
        (accum += remainder) % blockSize == 0 && this._update(block);
    }
    this._len += length;
    return this;
  }
  reset() {
    this._block.fill(0);
    this._len = 0;
    this._reset();
    return this;
  }

  digest(enc?: BufferEncoding) {
    var rem = this._len % this._blockSize;
    (this._block[rem] = 128),
      this._block.fill(0, rem + 1),
      rem >= this._finalSize &&
        (this._update(this._block), this._block.fill(0));
    var bits = 8 * this._len;
    if (bits <= 4294967295)
      this._block.writeUInt32BE(bits, this._blockSize - 4);
    else {
      var lowBits = (4294967295 & bits) >>> 0,
        highBits = (bits - lowBits) / 4294967296;
      this._block.writeUInt32BE(highBits, this._blockSize - 8),
        this._block.writeUInt32BE(lowBits, this._blockSize - 4);
    }
    this._update(this._block);
    var hash = this._hash();
    return enc ? hash.toString(enc) : hash;
  }
  _hash(): Buffer {
    throw new Error("_hash must be implemented by subclass");
  }
  _update(data: Buffer) {
    throw new Error("_update must be implemented by subclass");
  }
  _reset() {
    throw new Error("_reset must be implemented by subclass");
  }
}