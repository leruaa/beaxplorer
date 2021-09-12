import * as d3 from 'd3';

const formatTime = d3.timeFormat("%B %d, %Y");

d3.selectAll("span.timestamp")
  // @ts-ignore
  .datum(function () { return this.dataset; })
  .text((d) => formatTime(new Date(d.timestamp * 1000)));
