import { createClient, defaultExchanges, subscriptionExchange } from "urql";
import { SubscriptionClient } from "subscriptions-transport-ws";

const wsUri =
  (window.location.protocol === "https:" ? "wss://" : "ws://") +
  window.location.host +
  "/graphql/subscribe";

const subscriptionClient = new SubscriptionClient(wsUri, { reconnect: true });

export function changeToken(token) {
  if (subscriptionClient.connectionParams.authToken === token) {
    return;
  }
  localStorage.setItem("token", token);
  subscriptionClient.close();
  subscriptionClient.connect();
}

window.addEventListener("storage", (event) => {
  if (event.key === "token" && event.oldValue !== event.newValue) {
    changeToken(event.newValue);
  }
});

export const client = createClient({
  url: "/graphql",
  exchanges: [
    ...defaultExchanges,
    subscriptionExchange({
      forwardSubscription(operation) {
        return subscriptionClient.request(operation);
      },
    }),
  ],
  fetchOptions: () => {
    const token = localStorage.getItem("token");
    return {
      headers: { Authorization: token },
    };
  },
});
