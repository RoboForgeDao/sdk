export class RoboForgeSDK {
  programId: string;

  constructor(id: string) {
    this.programId = id;
  }

  async track(value: number) {
    console.log("track event", value);
  }
}
