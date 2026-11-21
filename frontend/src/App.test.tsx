import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { App } from "./App";
describe("Blastglass studio",()=>{
  it("renders a domain-specific rehearsal composer",()=>{
    vi.stubGlobal("fetch",vi.fn().mockResolvedValue({ok:true,json:async()=>[]}));
    render(<App/>);
    expect(screen.getByRole("heading",{name:/rehearse the outage/i})).toBeInTheDocument();
    expect(screen.getByRole("form",{name:/rehearsal composer/i})).toBeInTheDocument();
  });
});
