import ReactDOM from "react-dom/client";
import App from "./App";
import { metadata } from "@polkadot/types/interfaces/essentials";
import { KeyringPair } from "@polkadot/keyring/types";
import {
  ApiPromise,
  WsProvider,
  Keyring,
} from "@polkadot/api";
import "@polkadot/api-augment";

const WEB_SOCKET = "ws://localhost:9944";
// const sleep = (ms) =>
//   new Promise((resolve) =>
//     setTimeout(resolve, ms)
//   );

function delay(ms: number) {
  return new Promise((resolve) =>
    setTimeout(resolve, ms)
  );
}

//connect substrate chain

const connectSubstrate = async () => {
  const wsProvider = new WsProvider(WEB_SOCKET);
  const api = await ApiPromise.create({
    provider: wsProvider,
  });

  await api.isReady;
  console.log("substrate connected");

  return api;
};

//获取常量
const getConstants = async (api: ApiPromise) => {
  const existentialDeposit =
    await api.consts.balances.existentialDeposit.toHuman();
  return existentialDeposit;
};

//获取变量
const getFreeBalance = async (
  api: ApiPromise,
  address: string
) => {
  const aliceAccount =
    await api.query.system.account(address);
  return aliceAccount["data"]["free"].toHuman();
};

//获取动态数据
const printBalance = async (api: ApiPromise) => {
  const keyring = new Keyring({
    type: "sr25519",
  });
  const alice = keyring.addFromUri("//Alice");
  const bob = keyring.addFromUri("//Bob");

  console.log(
    "Alice's balance is:",
    await getFreeBalance(api, alice.address)
  );
  console.log(
    "Bob's balance is:",
    await getFreeBalance(api, bob.address)
  );
};

const subscribeAliceBalance = async (
  api: ApiPromise
) => {
  const keyring = new Keyring({
    type: "sr25519",
  });
  const alice = keyring.addFromUri("//Alice");
  await api.query.system.account(
    alice.address,
    (aliceAcc) => {
      console.log("subscribed to alice account");
      const aliceFreeAub = aliceAcc.data.free;
      console.log(
        `alice account(sub): ${aliceFreeAub}`
      );
    }
  );
};

const getMetadata = async (api: ApiPromise) => {
  const metadata =
    await api.rpc.state.getMetadata();
  console.log(metadata);
  return metadata;
};

//执行操作
const transFromAliceToBob = async (
  api: ApiPromise,
  amount: number
) => {
  const keyring = new Keyring({
    type: "sr25519",
  });

  const alice = keyring.addFromUri("//Alice");
  const bob = keyring.addFromUri("//Bob");

  await api.tx.balances
    .transfer(bob.address, amount)
    .signAndSend(alice, (res) => {
      console.log(`Tx status: ${res.status}`);
    });
};

//在main函数中调用connectSubstrate函数
const main = async () => {
  const api = await connectSubstrate();
  console.log(
    "const value existentialDeposit is:",
    await getConstants(api)
  );
  await printBalance(api);
  await transFromAliceToBob(api, 10 ** 12);
  await delay(6000);

  await printBalance(api);
  await subscribeAliceBalance(api);
  await delay(6000);

  await getMetadata(api);
  console.log("game over");
};

main()
  .then(() => {
    console.log("successfully exited");
    // process.exit(0);
  })
  .catch((err) => {
    console.log("error occur:", err);
    // process.exit(1);
  });

const root = ReactDOM.createRoot(
  document.getElementById("root") as HTMLElement
);
root.render(<App />);
