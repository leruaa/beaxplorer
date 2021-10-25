import { paginate } from "./pagination";

paginate("epochs", (rows) => {
  rows
    .select("td:nth-child(1) > a")
    .attr("href", (d: any) => `/epoch/${d.epoch}`)
    .text((d: any) => d.epoch);

  rows
    .select("td:nth-child(2)")
    .text((d: any) => d.ago);

  rows
    .select("td:nth-child(3)")
    .text((d: any) => d.attestations_count);

  rows
    .select("td:nth-child(4)")
    .text((d: any) => d.deposits_count);

  rows
    .select("td:nth-child(5)")
    .text((d: any) => `${d.proposer_slashings_count} / ${d.attester_slashings_count}`);

  rows
    .select("td:nth-child(6)")
    .text((d: any) => d.finalized ? "Yes" : "No");

  rows
    .select("td:nth-child(7)")
    .text((d: any) => `${d.eligible_ether} ETH`);

  rows
    .select("td:nth-child(8)")
    .text((d: any) => `${d.voted_ether} ETH`);

  rows
    .select("td:nth-child(9)")
    .text((d: any) => `${d.global_participation_percentage}%`);
});