import '../module-resolver-helper';

import { assert } from 'chai';
import { dataSchemaCreate, schemaCreate } from 'helpers/entities';
import {initVcxTestMode, shouldThrow, shouldThrowNapirs} from 'helpers/utils';
import { Schema, SchemaState, VCXCode } from 'src';

describe('Schema:', () => {
  before(() => initVcxTestMode());
  //
  describe('create:', () => {
    it('success', async () => {
      const schema = await schemaCreate();
    assert.equal(await schema.getState(), SchemaState.Published);
    });
  //
  //   it('throws: missing sourceId', async () => {
  //     const { sourceId, ...data } = dataSchemaCreate();
  //     const error = await shouldThrow(() => Schema.create(data as any));
  //     assert.equal(error.vcxCode, VCXCode.INVALID_OPTION);
  //   });
  //
  //   it('throws: missing data', async () => {
  //     const { data, ...rest } = dataSchemaCreate();
  //     const error = await shouldThrow(() => Schema.create(rest as any));
  //     assert.equal(error.vcxCode, VCXCode.UNKNOWN_ERROR);
  //   });
  //
  //   it('throws: imcpmplete data', async () => {
  //     const { data, ...rest } = dataSchemaCreate();
  //     const error = await shouldThrow(() => Schema.create({ data: {} as any, ...rest }));
  //     assert.equal(error.vcxCode, VCXCode.INVALID_OPTION);
  //   });
  //
  //   it('throws: missing data.name', async () => {
  //     const {
  //       data: { name, ...dataRest },
  //       ...rest
  //     } = dataSchemaCreate();
  //     const error = await shouldThrow(() => Schema.create({ data: dataRest, ...rest } as any));
  //     assert.equal(error.vcxCode, VCXCode.INVALID_OPTION);
  //   });
  //
  //   it('throws: missing data.version', async () => {
  //     const {
  //       data: { version, ...dataRest },
  //       ...rest
  //     } = dataSchemaCreate();
  //     const error = await shouldThrow(() => Schema.create({ data: dataRest, ...rest } as any));
  //     assert.equal(error.vcxCode, VCXCode.INVALID_OPTION);
  //   });
  //
  //   it('throws: missing data.attrNames', async () => {
  //     const {
  //       data: { attrNames, ...dataRest },
  //       ...rest
  //     } = dataSchemaCreate();
  //     const error = await shouldThrow(() => Schema.create({ data: dataRest, ...rest } as any));
  //     assert.equal(error.vcxCode, VCXCode.INVALID_OPTION);
  //   });
  //
  //   it('throws: invalid data', async () => {
  //     const { data, ...rest } = dataSchemaCreate();
  //     const error = await shouldThrow(() =>
  //       Schema.create({
  //         data: 'invalid' as any,
  //         ...rest,
  //       }),
  //     );
  //     assert.equal(error.vcxCode, VCXCode.INVALID_OPTION);
  //   });
  });

  describe('serialize:', () => {
    it('success', async () => {
      const schema = await schemaCreate();
      const serialized = await schema.serialize();
      assert.ok(serialized);
      assert.property(serialized, 'version');
      assert.property(serialized, 'data');
      const { data, version } = serialized;
      assert.ok(data);
      assert.ok(version);
      assert.equal(data.source_id, schema.sourceId);
    });

    // it('throws: not initialized', async () => {
    //   const schema = new Schema(null as any, {} as any);
    //   const error = await shouldThrow(() => schema.serialize());
    //   console.log(`Test Found error: ${JSON.stringify(error, null, 2)}`);
    //   // todo: remove this ts-ignore
    //   // @ts-ignore
    //     assert.equal(error.code, 'NumberExpected');
    // });
  });

  describe('deserialize:', () => {
    it('success', async () => {
      const schema1 = await schemaCreate();
      const data1 = await schema1.serialize();
      const schema2 = await Schema.deserialize(data1);
      assert.equal(schema2.sourceId, schema1.sourceId);
      const data2 = await schema2.serialize();
      assert.deepEqual(data1, data2);
    });

    it('throws: incorrect data', async () => {
      const error = await shouldThrowNapirs(async () =>
        Schema.deserialize({ data: { source_id: 'Invalid' } } as any),
      );
      assert.equal(error.vcxCode, VCXCode.INVALID_JSON);
    });
  });
});
