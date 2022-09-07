const { CwStargateReflectContract } = require("../dist/index");
const localState = require("../../../.beaker/state.local.json");
const { cosmos } = require("osmojs");
const { CosmWasmClient } = require("cosmwasm");

const { QueryParamsRequest } = cosmos.auth.v1beta1;

const contractAddress =
  localState.local["cw-stargate-reflect"].addresses.default;

let client;

beforeAll(async () => {
  client = new CwStargateReflectContract.CwStargateReflectQueryClient(
    await CosmWasmClient.connect("http://localhost:26657"),
    contractAddress
  );
});

const stargateResultToJSON = (res) => {
  let buff = Buffer.from(res.value, "base64");
  let text = buff.toString("utf8");

  return JSON.parse(text);
};

const queryStargate = async ({ path, request, encoder }) => {
  const queryRequestBytes = encoder.encode(request).finish();
  const queryRequest = Buffer.from(queryRequestBytes).toString("base64");

  const res = await client.queryStargate({
    path,
    queryRequest,
  });
  return stargateResultToJSON(res);
};

test("/cosmos.auth.v1beta1.Query/Params", async () => {
  const res = await queryStargate({
    path: "/cosmos.auth.v1beta1.Query/Params",
    request: {},
    encoder: QueryParamsRequest,
  });
  expect(res).toEqual({
    params: {
      max_memo_characters: "256",
      tx_sig_limit: "7",
      tx_size_cost_per_byte: "10",
      sig_verify_cost_ed25519: "590",
      sig_verify_cost_secp256k1: "1000",
    },
  });
});
