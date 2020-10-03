import React from "react";
import { render } from "@testing-library/react";
import App from "./App";

test("renders playground link", () => {
  const { getByText } = render(<App />);
  const linkElement = getByText(/open playground/i);
  expect(linkElement).toBeInTheDocument();
});
