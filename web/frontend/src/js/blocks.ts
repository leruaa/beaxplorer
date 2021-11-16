import { html } from "gridjs";
import { paginate } from "./pagination";

paginate({
  wrapperId: "blocks-list",
  apiUrl: "/api/blocks",
  columns: [
    {
      id: "epoch",
      name: "Epoch",
      formatter: (cell: any) => html(`<a href="/epoch/${cell}">${cell}</a>`)
    },
    {
      id: "slot",
      name: "Slot",
      formatter: (cell: any) => html(`<a href="/block/${cell}">${cell}</a>`)
    },
    {
      id: "ago",
      name: "Time"
    },
    {
      id: "proposer",
      name: "Proposer",
      formatter: (cell: any) => html(`<a href="/validator/${cell}">${cell}</a>`)
    },
    {
      id: "attestations_count",
      name: "Attestations"
    }
  ],
  dataMapping: (data: any) => data.results.map(
    (e: any) => [
      e.epoch,
      e.slot,
      e.ago,
      e.proposer,
      e.attestations_count
    ]
  )
});
