export class Lock {
  constructor() {
    this.lockHolder = null;
  }

  async withLock(scope) {
    while (this.lockHolder !== null) {
      await this.lockHolder;
    }
    this.lockHolder = Promise.resolve(null).then(scope);
    await this.lockHolder;
    this.lockHolder = null;
  }
}
