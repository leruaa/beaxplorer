import * as d3 from 'd3';

export function paginate() {
  let currentPage = 1;

  d3.select("#next-page").on("click", () => {
    fetch("/api/epochs?page=" + (currentPage + 1))
      .then((response) => {
        return response.json();
      })
      .then((json) => {

        d3.select("#epochs").selectAll("tbody > tr")
          .data(json)
          .join("tr")
          .selectAll("td")
          .data((d: any) => [d.epoch, d.ago, d.attestations_count, d.deposits_count, "/", d.finalized, d.eligible_ether, d.voted_ether, d.global_participation_percentage])
          .join("td")
          .text((d: any) => d);

        currentPage++;
      });
  });
}

