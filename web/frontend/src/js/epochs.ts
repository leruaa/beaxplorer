import { Grid, html } from "gridjs";

new Grid({
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
  server: {
    url: '/api/epochs',
    then: data => data.map(
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
  }
}).render(document.getElementById("epochs-list")!);

/*
paginate("epochs",
  (row) => {
    row
      .append("td")
      .append("a")
      .attr("href", (d: any) => `/epoch/${d.epoch}`)
      .text((d: any) => d.epoch);

    row
      .append("td")
      .text((d: any) => d.ago);

    row
      .append("td")
      .text((d: any) => d.attestations_count);

    row
      .append("td")
      .text((d: any) => d.deposits_count);

    row
      .append("td")
      .text((d: any) => `${d.proposer_slashings_count} / ${d.attester_slashings_count}`);

    row
      .append("td")
      .text((d: any) => d.finalized ? "Yes" : "No");

    row
      .append("td")
      .text((d: any) => `${d.eligible_ether} ETH`);

    row
      .append("td")
      .text((d: any) => `${d.voted_ether} ETH`);

    row
      .append("td")
      .text((d: any) => `${d.global_participation_percentage}%`);

  },
  (row) => {
    row
      .select("td:nth-child(1) > a")
      .attr("href", (d: any) => `/epoch/${d.epoch}`)
      .text((d: any) => d.epoch);

    row
      .select("td:nth-child(2)")
      .text((d: any) => d.ago);

    row
      .select("td:nth-child(3)")
      .text((d: any) => d.attestations_count);

    row
      .select("td:nth-child(4)")
      .text((d: any) => d.deposits_count);

    row
      .select("td:nth-child(5)")
      .text((d: any) => `${d.proposer_slashings_count} / ${d.attester_slashings_count}`);

    row
      .select("td:nth-child(6)")
      .text((d: any) => d.finalized ? "Yes" : "No");

    row
      .select("td:nth-child(7)")
      .text((d: any) => `${d.eligible_ether} ETH`);

    row
      .select("td:nth-child(8)")
      .text((d: any) => `${d.voted_ether} ETH`);

    row
      .select("td:nth-child(9)")
      .text((d: any) => `${d.global_participation_percentage}%`);
  }
);
*/