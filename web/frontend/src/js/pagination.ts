import * as d3 from 'd3';

export interface Refresh {
  (rows: d3.Selection<d3.BaseType | HTMLTableRowElement, unknown, d3.BaseType, unknown>): void;
}

export function paginate(type: string, refresh: Refresh) {
  let currentPage = 1;

  d3.select("#next-page").on("click", () => {
    fetch(`/api/${type}?page=${currentPage + 1}`)
      .then((response) => {
        return response.json();
      })
      .then((json) => {

        let rows = d3.select("#" + type).selectAll("tbody > tr")
          .data(json)
          .join("tr");

        refresh(rows);

        currentPage++;
      });
  });
}

