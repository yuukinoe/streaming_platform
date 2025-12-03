<script lang="ts">
	import type { ApexOptions } from 'apexcharts';
	// @ts-ignore
	import { Avatar } from 'flowbite-svelte';
	import { Chart } from '@flowbite-svelte-plugins/chart';
	import { invertColor, pastelColors_2, shuffle } from '$lib/utils.ts';
	import { fade } from 'svelte/transition';
	// Get data from props
	let { data } = $props();

	let selectMode = $state('compact');

	// Function to generate last 30 days
	function generateLast30Days(): string[] {
		const dates: string[] = [];
		const today = new Date();

		for (let i = 30; i >= 0; i--) {
			const date = new Date(today);
			date.setDate(today.getDate() - i);
			dates.push(date.toISOString().split('T')[0]); // Format as YYYY-MM-DD
		}

		return dates;
	}

	// Function to process analytics data from the full JSON structure
	function processFullAnalyticsData(fullData: any[]) {
		const last30Days = generateLast30Days();
		const animeMap = new Map<string, Map<string, number>>();

		// Process analytics data
		fullData.forEach((entry) => {
			if (entry.kind === 'anime' && entry.object_id && entry.object_id.title) {
				const animeTitle = entry.object_id.title;
				const date = entry.date;
				const views = entry.views || 0;

				if (!animeMap.has(animeTitle)) {
					animeMap.set(animeTitle, new Map());
				}

				const animeData = animeMap.get(animeTitle)!;
				// If there's already data for this date, add the views
				const existingViews = animeData.get(date) || 0;
				animeData.set(date, existingViews + views);
			}
		});

		// Generate series data
		const series: any[] = [];
		const colors = shuffle(pastelColors_2);

		animeMap.forEach((animeData, animeTitle) => {
			const data: number[] = [];

			last30Days.forEach((date) => {
				data.push(animeData.get(date) || 0);
			});

			series.push({
				name: animeTitle,
				data: data,
				color: colors[series.length % colors.length]
			});
		});

		return {
			categories: last30Days.map((date) => {
				const d = new Date(date);
				return d.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
			}),
			series: series
		};
	}

	// function to make a chart for the overall analytics it will make a dates from 30 days ago to today and will match it in list with data.overallAnalytics if there is not data for the date it will be 0 i only need function to return list with numbers and categories with dates
	function processOverallAnalyticsData(overallData: any[]) {
		const last30Days = generateLast30Days();
		const dateMap = new Map<string, number>();

		// Process overall analytics data
		overallData.forEach((entry) => {
			const date = entry.date;
			const views = entry.views || 0;

			// If there's already data for this date, add the views
			const existingViews = dateMap.get(date) || 0;
			dateMap.set(date, existingViews + views);
		});

		// Generate data array for the last 30 days
		const data: number[] = [];

		last30Days.forEach((date) => {
			data.push(dateMap.get(date) || 0);
		});

		return {
			categories: last30Days.map((date) => {
				const d = new Date(date);
				return d.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
			}),
			series: data
		};
	}

	// Process the data from props
	const chartData = processFullAnalyticsData(data.animeAnalytics || []);
	const chartDataOverall = processOverallAnalyticsData(data.overallAnalytics || []);
	let options: ApexOptions = {
		chart: {
			height: 'auto',
			width: '100%',
			type: 'line',
			fontFamily: 'roboto, sans-serif',
			dropShadow: {
				enabled: false
			},
			toolbar: {
				show: false
			},
			animations: {
				enabled: false
			},
			zoom: {
				enabled: false
			}
		},
		tooltip: {
			enabled: true,
			shared: false,
			intersect: true,
			followCursor: true,
			custom: function ({ series, seriesIndex, dataPointIndex, w }) {
				// Get the hovered series data
				const hoveredSeries = series[seriesIndex];
				const hoveredValue = hoveredSeries[dataPointIndex];
				const category = w.globals.labels[dataPointIndex];

				// Only show tooltip if there's actual data for this series at this point
				if (hoveredValue && hoveredValue > 0) {
					// Find all series with the same value at this point
					const seriesWithSameData = [];

					for (let i = 0; i < series.length; i++) {
						const value = series[i][dataPointIndex];
						if (value === hoveredValue && value > 0) {
							seriesWithSameData.push({
								seriesIndex: i,
								name: w.globals.seriesNames[i],
								value: value,
								color: w.globals.colors[i]
							});
						}
					}

					// Build tooltip HTML for all matching series
					let tooltipHtml = '';
					seriesWithSameData.forEach((item, index) => {
						tooltipHtml += `
							<div class="flex justify-between gap-10 font-roboto" style="background: ${item.color}; color: ${invertColor(item.color)}">
								<div style="font-weight: bold;" class="text-sm">${item.name}</div>
								<div class="text-xs">${item.value} views</div>
							</div>
						`;
					});
					let tooltipDIV = `
					<div class="flex flex-col border-2 border-teal-600">
					${tooltipHtml}
					</div>
					
					`;
					return tooltipDIV;
				}

				return '';
			},
			x: {
				show: true
			}
		},
		dataLabels: {
			enabled: false
		},
		stroke: {
			width: 5,
			curve: 'smooth'
		},
		markers: {
			size: 4,
			hover: {
				sizeOffset: 6
			}
		},
		states: {
			hover: {
				filter: {
					type: 'lighten'
				}
			},
			active: {
				filter: {
					type: 'darken'
				}
			}
		},
		grid: {
			show: true,
			strokeDashArray: 4,
			padding: {
				left: 2,
				right: 2,
				top: -26
			}
		},
		series: chartData.series,
		legend: {
			show: false,
			position: 'top',
			horizontalAlign: 'left',
			fontSize: '12px'
		},
		xaxis: {
			categories: chartData.categories,
			labels: {
				show: true,
				style: {
					fontFamily: 'roboto, sans-serif',
					cssClass: 'text-xs font-normal fill-gray-500 dark:fill-gray-400'
				}
			},
			axisBorder: {
				show: false
			},
			axisTicks: {
				show: false
			}
		},
		yaxis: {
			show: true,
			labels: {
				style: {
					fontFamily: 'roboto, sans-serif',
					cssClass: 'text-xs font-normal fill-gray-500 dark:fill-gray-400'
				}
			}
		}
	};
	let optionsOverall: ApexOptions = {
		chart: {
			height: 'auto',
			type: 'area',
			fontFamily: 'Inter, sans-serif',
			dropShadow: {
				enabled: false
			},
			toolbar: {
				show: false
			}
		},
		tooltip: {
			enabled: true,
			x: {
				show: false
			}
		},
		series: [
			{
				name: 'Users',
				data: chartDataOverall.series,
				color: '#1A56DB'
			},
		]
	};
</script>

<div class="w-full">
	<div class="w-full flex justify-center items-center p-3">
		<div class="flex gap-5 justify-between items-center">
			<Avatar src={data.isLoggedIn.message.avatar} alt="avatar" class="rounded-full" size="xl" />
			<div class="flex flex-col">
				<span class="text-white font-roboto text-3xl">Willkommen, {data.isLoggedIn.message.name}</span>
				<span class="text-gray-400 font-roboto">{data.isLoggedIn.message.roles ? (data.isLoggedIn.message.roles.length >= 1 ? data.isLoggedIn.message.roles[0].name : '') : ''}{data.isLoggedIn.message.is_superuser ? ' - Superuser' : ''}</span>
			</div>
		</div>
	</div>
	<div transition:fade class={`grid gap-1 p-4 md:p-6 ${selectMode === 'compact' ? 'grid-cols-2' : 'grid-cols-1'}`}>
		<div class="[&>*:nth-child(2)]:h-full">
			<div class="flex justify-between w-full">
				<span class="text-xl font-semibold mb-7 text-white font-roboto">Anime Views Analytics (Last 30 Days)</span>
				<select bind:value={selectMode} class="bg-gray-700 border border-gray-600 text-white text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block p-1 h-8 font-roboto">
					<option value="compact">Compact</option>
					<option value="full">Full</option>
				</select>
			</div>
			<Chart {options} />
			<!-- <div class="mt-2.5 grid grid-cols-1 items-center justify-between border-t border-gray-200 dark:border-gray-700">
				<div class="pt-5"></div>
			</div> -->
		</div>

		<div class="[&>div]:h-full">
			<span class="text-xl font-semibold mb-7 text-white font-roboto">Overall Analytics (Last 30 Days)</span>
			<Chart options={optionsOverall} />
			<!-- <div class="mt-2.5 grid grid-cols-1 items-center justify-between border-t border-gray-200 dark:border-gray-700">
				<div class="pt-5"></div>
			</div> -->
		</div>
	</div>
</div>
