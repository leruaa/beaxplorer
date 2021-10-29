import * as d3 from 'd3';

export interface Refresh {
  (rows: d3.Selection<d3.BaseType | HTMLTableRowElement, unknown, d3.BaseType, unknown>): void;
}

export function paginate(type: string, refresh: Refresh) {
  const firstPage = d3.select("#first-page");
  const previousPage = d3.select("#previous-page");
  const currentPage = d3.select("#current-page");
  const pageCount = d3.select("#page-count");
  const nextPage = d3.select("#next-page");
  const lastPage = d3.select("#last-page");

  let currentPageIndex = 1;
  let totalPageCount = parseInt(pageCount.attr("data-page-count"));

  const update = (newPageIndex: number) => {
    fetch(`/api/${type}?page=${newPageIndex}`)
      .then((response) => {
        return response.json();
      })
      .then((json) => {

        let rows = d3.select("#" + type).selectAll("tbody > tr")
          .data(json)
          .join("tr");

        refresh(rows);

        firstPage.attr("disabled", newPageIndex == 1 ? "disabled" : null);
        previousPage.attr("disabled", newPageIndex == 1 ? "disabled" : null);
        currentPage.attr("value", newPageIndex);
        nextPage.attr("disabled", newPageIndex == totalPageCount ? "disabled" : null);
        lastPage.attr("disabled", newPageIndex == totalPageCount ? "disabled" : null);
        currentPageIndex = newPageIndex;
      });
  }

  firstPage.on("click", () => update(1));
  nextPage.on("click", () => update(currentPageIndex + 1));
  previousPage.on("click", () => update(currentPageIndex - 1));
  lastPage.on("click", () => update(totalPageCount));
}

