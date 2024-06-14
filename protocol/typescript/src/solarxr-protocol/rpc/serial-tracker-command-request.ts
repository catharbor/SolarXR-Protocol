// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';



/**
 * Sends arbitrary command to the current tracker on the serial monitor
 */
export class SerialTrackerCommandRequest implements flatbuffers.IUnpackableObject<SerialTrackerCommandRequestT> {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):SerialTrackerCommandRequest {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsSerialTrackerCommandRequest(bb:flatbuffers.ByteBuffer, obj?:SerialTrackerCommandRequest):SerialTrackerCommandRequest {
  return (obj || new SerialTrackerCommandRequest()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsSerialTrackerCommandRequest(bb:flatbuffers.ByteBuffer, obj?:SerialTrackerCommandRequest):SerialTrackerCommandRequest {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new SerialTrackerCommandRequest()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

command():string|null
command(optionalEncoding:flatbuffers.Encoding):string|Uint8Array|null
command(optionalEncoding?:any):string|Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.__string(this.bb_pos + offset, optionalEncoding) : null;
}

static startSerialTrackerCommandRequest(builder:flatbuffers.Builder) {
  builder.startObject(1);
}

static addCommand(builder:flatbuffers.Builder, commandOffset:flatbuffers.Offset) {
  builder.addFieldOffset(0, commandOffset, 0);
}

static endSerialTrackerCommandRequest(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createSerialTrackerCommandRequest(builder:flatbuffers.Builder, commandOffset:flatbuffers.Offset):flatbuffers.Offset {
  SerialTrackerCommandRequest.startSerialTrackerCommandRequest(builder);
  SerialTrackerCommandRequest.addCommand(builder, commandOffset);
  return SerialTrackerCommandRequest.endSerialTrackerCommandRequest(builder);
}

unpack(): SerialTrackerCommandRequestT {
  return new SerialTrackerCommandRequestT(
    this.command()
  );
}


unpackTo(_o: SerialTrackerCommandRequestT): void {
  _o.command = this.command();
}
}

export class SerialTrackerCommandRequestT implements flatbuffers.IGeneratedObject {
constructor(
  public command: string|Uint8Array|null = null
){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  const command = (this.command !== null ? builder.createString(this.command!) : 0);

  return SerialTrackerCommandRequest.createSerialTrackerCommandRequest(builder,
    command
  );
}
}
