export function show_chart(data) {
  console.log("JavaScript received data:")
  console.log(data)

  const xValue = d => Date.parse(d.x);
  const xLabel = 'Time';
  const yValue = d => d.y;
  const yLabel = 'Temperature';
  const margin = { left: 120, right: 30, top: 20, bottom: 120 };

  const svg = d3.select('#chart');
  const width = svg.attr('width');
  const height = svg.attr('height');
  const innerWidth = width - margin.left - margin.right;
  const innerHeight = height - margin.top - margin.bottom;

  const g = svg.append('g')
    .attr('transform', `translate(${margin.left},${margin.top})`);
  const xAxisG = g.append('g')
    .attr('transform', `translate(0, ${innerHeight})`);
  const yAxisG = g.append('g');

  xAxisG.append('text')
    .attr('class', 'axis-label')
    .attr('x', innerWidth / 2)
    .attr('y', 100)
    .text(xLabel);

  yAxisG.append('text')
    .attr('class', 'axis-label')
    .attr('x', -innerHeight / 2)
    .attr('y', -60)
    .attr('transform', `rotate(-90)`)
    .style('text-anchor', 'middle')
    .text(yLabel);

  const xScale = d3.scaleTime();
  const yScale = d3.scaleLinear();

  const xAxis = d3.axisBottom()
    .scale(xScale)
    .tickPadding(15)
    .tickSize(-innerHeight);

  const yAxis = d3.axisLeft()
    .scale(yScale)
    .ticks(5)
    .tickPadding(15)
    .tickSize(-innerWidth);

  xScale
    .domain(d3.extent(data, xValue))
    .range([0, innerWidth])
    .nice();

  yScale
    .domain([0, 4]/* d3.extent(data, yValue) */)
    .range([innerHeight, 0])
    .nice();

  g.selectAll('circle').data(data)
    .enter().append('circle')
    .attr('cx', d => xScale(xValue(d)))
    .attr('cy', d => yScale(yValue(d)))
    .attr('fill-opacity', 0.6)
    .attr('r', 8);

  xAxisG.call(xAxis);
  yAxisG.call(yAxis);
}
