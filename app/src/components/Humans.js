import React from "react";
import { useQuery } from "urql";

const HumansQuery = `
  query {
    humans {
      id
      name
      appearsIn
      homePlanet
    }
  }
`;

function Humans() {
  const [result, reexecuteQuery] = useQuery({ query: HumansQuery });
  const { data, fetching, error } = result;

  if (fetching) return <p>Loading...</p>;
  if (error) return <p>Oh no... {error.message}</p>;

  const refresh = () => reexecuteQuery({ requestPolicy: "network-only" });

  return (
    <div style={{ border: "1px solid grey", padding: 16, marginBottom: 12 }}>
      {data.humans.length ? (
        <ul>
          {data.humans.map((human) => (
            <li key={human.id}>
              <strong>{human.name}</strong> in {human.homePlanet}
              {human.appearsIn && ` (appears in ${human.appearsIn})`}
            </li>
          ))}
        </ul>
      ) : (
        <p>No humans yet!</p>
      )}
      <button onClick={refresh} style={{ padding: 8, fontSize: 16 }}>
        Refresh Query
      </button>
    </div>
  );
}

export default Humans;
