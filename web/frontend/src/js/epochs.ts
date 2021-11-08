import { html } from "gridjs";
import { paginate } from "./pagination";

paginate({
  wrapperId: "epochs-list",
  apiUrl: "/api/epochs",
  columns: [
    {
      name: "Epoch",
      formatter: (cell: any) => html(`<a href="/epoch/${cell}">${cell}</a>`)
    },
    "Time",
    "Attestations",
    "Deposits",
    "Slashings P / A",
    "Finalized",
    {
      name: "Eligible",
      formatter: (cell: any) => `${cell} ETH`
    },
    {
      name: "Voted",
      formatter: (cell: any) => `${cell} ETH`
    },
    {
      name: "Rate",
      formatter: (cell: any) => `${cell}%`
    }
  ],
  dataMapping: (data: any) => data.map(
    (e: any) => [
      e.epoch,
      e.ago,
      e.attestations_count,
      e.deposits_count,
      `${e.proposer_slashings_count} / ${e.attester_slashings_count}`,
      `${e.finalized ? "Yes" : "No"}`,
      e.eligible_ether,
      e.voted_ether,
      e.global_participation_percentage
    ]
  )
});
