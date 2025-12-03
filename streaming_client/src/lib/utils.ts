export function getSeasonDates(): [Date, Date] {
	const now = new Date();
	const year = now.getFullYear();

	let start: Date;
	let end: Date;

	if (new Date(year, 3, 1) > now) {
		// Winter: Jan 1 – Mar 31
		start = new Date(year, 0, 1);
		end = new Date(year, 2, 31);
	} else if (new Date(year, 6, 1) > now) {
		// Spring: Apr 1 – Jun 30
		start = new Date(year, 3, 1);
		end = new Date(year, 5, 30);
	} else if (new Date(year, 9, 1) > now) {
		// Summer: Jul 1 – Sep 30
		start = new Date(year, 6, 1);
		end = new Date(year, 8, 30);
	} else {
		// Fall: Oct 1 – Dec 31
		start = new Date(year, 9, 1);
		end = new Date(year, 11, 31);
	}

	return [start, end];
}

export function getSeasonName(): string {
	const now = new Date();
	const year = now.getFullYear();

	if (new Date(year, 3, 1) > now) {
		// Winter season
		return `Anime zima ${year}`;
	} else if (new Date(year, 6, 1) > now) {
		// Spring season
		return `Anime wiosna ${year}`;
	} else if (new Date(year, 9, 1) > now) {
		// Summer season
		return `Anime lato ${year}`;
	} else {
		// Autumn season
		return `Anime jesień ${year}`;
	}
}

export function getSeasonFromDate(dateStr: string): string {
	const date = new Date(dateStr);
	const year = date.getFullYear();

	if (date < new Date(year, 3, 1)) {
		return `Zima ${year}`;
	} else if (date < new Date(year, 6, 1)) {
		return `Wiosna ${year}`;
	} else if (date < new Date(year, 9, 1)) {
		return `Lato ${year}`;
	} else {
		return `Jesień ${year}`;
	}
}

export function getDomainName(url: string): string {
	try {
		const parsedUrl = new URL(url);
		const hostname = parsedUrl.hostname.replace('www.', '');
		const parts = hostname.split('.');
		if (parts.length >= 2) {
			return parts[parts.length - 2].toUpperCase().replace('GOOGLE', 'GDRIVE').replace("RPMVID", "RPMV")
				.replace("VIDTUBE", "VT")
				.replace("MOVEARNPRE", "MAP");
		}
		let r = hostname.toUpperCase().replace('GOOGLE', 'GDRIVE').replace("RPMVID", "RPMV")
			.replace("VIDTUBE", "VT")
			.replace("MOVEARNPRE", "MAP");
		return r;
	} catch (error) {
		console.error('Invalid URL:', url, error);
		return '';
	}
}

export function servicesEmbedSupport(service: string) {
	if (service.includes('vk.com')) {
		const vk_regex = /video(\-?\d+)_+(\d+)/;
		const match = service.match(vk_regex);
		if (match) {
			const [, oid, id] = match;
			return `https://vk.com/video_ext.php?oid=${oid}&id=${id}`;
		}
	}
	if (!service.includes('https://')) {
		service = 'https://' + service;
	}
	if (service.includes('drive.google')) {
		return service.replace('/view', '/preview')
	}
	return service
		.replace('https://cda.pl/video/', 'https://ebd.cda.pl/620x395/')
		.replace('https://www.cda.pl/video/', 'https://ebd.cda.pl/620x395/')
		.replace('https://mega.nz/file/', 'https://mega.nz/embed/')
		.replace('/view?usp=sharing', '/preview').replace("https://ryderjet.com/file", "https://ryderjet.com/embed");
}



export const hexToRGB = (hex: string) => {
	hex = hex.startsWith('#') ? hex.slice(1) : hex;
	if (hex.length === 3) {
		hex = Array.from(hex).reduce((str, x) => str + x + x, ''); // 123 -> 112233
	}
	let values = hex
		.split(/([a-z0-9]{2,2})/)
		.filter(Boolean)
		.map((x) => parseInt(x, 16));
	if (values.length === 4) {
		values = values.slice(0, 3);
	}
	return `rgb(${values.join(' ')} / 43%)`;
};

export function getTimeAgo(dateString: string) {
	const datePosted = new Date(dateString);
	const now = new Date();
	const diffMs = now.getTime() - datePosted.getTime();

	const diffSeconds = Math.floor(diffMs / 1000);
	const diffMinutes = Math.floor(diffMs / (1000 * 60));
	const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
	const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));

	if (diffDays >= 1) {
		return `${diffDays} dni temu`;
	} else if (diffHours >= 1) {
		return `${diffHours} godzin temu`;
	} else if (diffMinutes >= 1) {
		return `${diffMinutes} minut temu`;
	} else {
		return `${diffSeconds} sekund temu`;
	}
}

export function stripHTML(input: string) {
	const doc = new DOMParser().parseFromString(input, 'text/html');
	return doc.body.textContent || '';
}



export function stripHTMLWithoutDomParser(input: string) {
	const withoutScriptsAndStyles = input
		.replace(/<script[^>]*>[\s\S]*?<\/script>/gi, '')
		.replace(/<style[^>]*>[\s\S]*?<\/style>/gi, '');

	// Usuń wszystkie tagi HTML
	const withoutTags = withoutScriptsAndStyles.replace(/<[^>]+>/g, ' ');

	// Dekoduj encje HTML
	if (typeof globalThis.document !== 'undefined') {
		const textarea = document.createElement('textarea');
		textarea.innerHTML = withoutTags;
		return textarea.value.trim();
	}

	// Fallback serwerowy – minimalna dekodowanie encji
	return withoutTags
		.replace(/&nbsp;/g, ' ')
		.replace(/&amp;/g, '&')
		.replace(/&lt;/g, '<')
		.replace(/&gt;/g, '>')
		.trim();
}

export function formatToRustDatetime(date: Date): string {
	const year = date.getFullYear();
	const month = String(date.getMonth() + 1).padStart(2, '0');
	const day = String(date.getDate()).padStart(2, '0');
	const hours = String(date.getHours()).padStart(2, '0');
	const minutes = String(date.getMinutes()).padStart(2, '0');
	const seconds = String(date.getSeconds()).padStart(2, '0');

	return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
}
export function formatDateToISOWithNanoseconds(date: Date): string {
	const pad = (num: number, size: number) => num.toString().padStart(size, '0');

	const year = date.getFullYear();
	const month = pad(date.getMonth() + 1, 2);
	const day = pad(date.getDate(), 2);
	const hour = pad(date.getHours(), 2);
	const minute = pad(date.getMinutes(), 2);
	const second = pad(date.getSeconds(), 2);

	const milliseconds = pad(date.getMilliseconds(), 3);
	const nanoseconds = milliseconds + '000000'; // pad ms to 9 digits

	return `${year}-${month}-${day}T${hour}:${minute}:${second}.${nanoseconds}`;
}

export const pastelColors = [
	// Pink Pastels (20)
	"#FFB6C1", "#FFC0CB", "#FFCCCB", "#FFD1DC", "#FFE4E6", "#FFEEF0", "#FFB3BA", "#FFC1C8",
	"#FFD0D6", "#FFDEDF", "#FFECEC", "#FFF0F0", "#FFE1E6", "#FFC2D1", "#FFB7CC", "#FFACC7",
	"#FFA1C2", "#FF96BD", "#FF8BB8", "#FFDEE9",
	// Blue Pastels (20)
	"#ADD8E6", "#B0E0E6", "#AFEEEE", "#E0FFFF", "#F0FFFF", "#B8E6FF", "#C7EFFF", "#D6F8FF",
	"#E5FFFF", "#F4FFFF", "#AED6F1", "#BBE2F9", "#C8E8FA", "#D5EEFB", "#E2F4FC", "#EFFAFD",
	"#A7D0EE", "#B4D6F0", "#C1DCF2", "#CEE2F4",
	// Purple Pastels (20)
	"#DDA0DD", "#E6E6FA", "#F8F8FF", "#E6D3FF", "#F0E6FF", "#F5F0FF", "#FAFAFF", "#D8BFD8",
	"#E0C7E0", "#E8CFE8", "#F0D7F0", "#F8DFF8", "#FFE7FF", "#FFEFFF", "#DCC4DC", "#E4CCE4",
	"#ECD4EC", "#F4DCF4", "#FCE4FC", "#FFECFF",
	// Green Pastels (20)
	"#98FB98", "#90EE90", "#F0FFF0", "#E0FFE0", "#D0FFD0", "#C0FFC0", "#B0FFB0", "#A0FFA0",
	"#98F098", "#B8F8B8", "#D8FFD8", "#E8FFE8", "#F8FFF8", "#A0E0A0", "#B0E8B0", "#C0F0C0",
	"#D0F8D0", "#E0FFE0", "#F0FFF0", "#F8FFF8",
	// Red Pastels (20)
	"#FFB3B3", "#FFC1C1", "#FFD0D0", "#FFDDDD", "#FFE6E6", "#FFCCCC", "#FFD7D7", "#FFE1E1",
	"#FFEBEB", "#FFF0F0", "#FFAAA", "#FFBFBF", "#FFCDCD", "#FFDADA", "#FFE7E7", "#FFCACA",
	"#FFD5D5", "#FFDFDF", "#FFE9E9", "#FFF2F2",
	// Yellow Pastels (20)
	"#FFFFE0", "#FFFACD", "#FFF8DC", "#F0E68C", "#FAFAD2", "#FFFF99", "#FFFFB3", "#FFFFCC",
	"#FFFFE6", "#FFFFF0", "#FFFFF5", "#FFFFFA", "#FFFFC0", "#FFFFD0", "#FFFFE0", "#FFFFF0",
	"#FFFFB0", "#FFFFC5", "#FFFFD5", "#FFFFE5",
	// Orange Pastels (20)
	"#FFEEE6", "#FFE4CC", "#FFDAB3", "#FFD099", "#FFC680", "#FFBC66", "#FFB24D", "#FFA833",
	"#FF9E1A", "#FF9400", "#FFEBCC", "#FFE1B3", "#FFD799", "#FFCD80", "#FFC366", "#FFB94D",
	"#FFAF33", "#FFA51A", "#FF9B00", "#FF9100",
	// Mint Pastels (20)
	"#F0FFFF", "#E0FFFF", "#AFEEEE", "#B8FFF8", "#C0FFF0", "#D0FFF8", "#E0FFFA", "#F0FFFC",
	"#F8FFFE", "#FCFFFF", "#B0F8F8", "#C8FFFF", "#D8FFFF", "#E8FFFF", "#F4FFFF", "#FAFFFF",
	"#B5F8F8", "#C5FFFF", "#D5FFFF", "#E5FFFF",
	// Coral/Peach Pastels (20)
	"#FFDAB9", "#FFE4E1", "#FFCCCB", "#FFCDD2", "#FFCC99", "#FFB07A", "#FFC0A4", "#FFD1B3",
	"#FFE4CC", "#FFF0E6", "#FFDDC1", "#FFE4C4", "#FFECD6", "#FFF3E0", "#FFF8E7", "#FFDEC5",
	"#FFE5C7", "#FFEDD8", "#FFF4E2", "#FFF9E9",
	// Lavender Pastels (20)
	"#E6E6FA", "#F8F8FF", "#F0F8FF", "#E0E6FF", "#F5F5FF", "#EDE7F6", "#F3E5F5", "#E8EAF6",
	"#FAF0E6", "#F5F5DC", "#E4E4FA", "#ECECFC", "#F4F4FE", "#FCFCFF", "#E8E8FC", "#F0F0FE",
	"#F8F8FF", "#ECECFE", "#F4F4FF", "#FCFCFF",
	// Teal Pastels (20)
	"#E0FEFE", "#B0E0E6", "#AFEEEE", "#C4E8E8", "#D0F0F0", "#E0F8F8", "#F0FCFC", "#E6FFFF",
	"#F5FFFF", "#FAFFFF", "#B8E8E8", "#C8F0F0", "#D8F8F8", "#E8FCFC", "#F4FEFE", "#FAFFFF",
	"#BCE8E8", "#CCF0F0", "#DCF8F8", "#ECFCFC",
	// Cream/Beige Pastels (20)
	"#FFF8DC", "#FFFACD", "#F5F5DC", "#FAF0E6", "#FDF5E6", "#FFFAF0", "#FFF5EE", "#F0F8FF",
	"#FFFFF0", "#FFFFFB", "#FFF9E6", "#FFFBEA", "#FFFDEF", "#FFFFF3", "#FFFFF8", "#FFFFFC",
	"#FFF7E0", "#FFF9E4", "#FFFBE8", "#FFFDEC",
	// Rose Pastels (20)
	"#FFE4E1", "#FFF0F5", "#FFEBEE", "#FFD1DC", "#FFC0CB", "#FFB6C1", "#FFDAB9", "#FFE4B5",
	"#FFEFD5", "#FFF8DC", "#FFE1E6", "#FFE7EA", "#FFEDED", "#FFF3F4", "#FFF9FA", "#FFFCFC",
	"#FFDEDF", "#FFE4E5", "#FFEAEB", "#FFF0F1",
	// Sage Green Pastels (20)
	"#C8E6C8", "#D0E8D0", "#D8F0D8", "#E0F8E0", "#E8FFF8", "#F0FFFF", "#F8FFFF", "#B8E0B8",
	"#C0E8C0", "#C8F0C8", "#D0F8D0", "#D8FFD8", "#E0FFE0", "#E8FFE8", "#B0D8B0", "#B8E0B8",
	"#C0E8C0", "#C8F0C8", "#D0F8D0", "#D8FFD8",
	// Sky Blue Pastels (20)
	"#E0F6FF", "#E8F8FF", "#F0FAFF", "#F8FCFF", "#FCFEFF", "#FFFFFF", "#D8F4FF", "#E0F6FF",
	"#E8F8FF", "#F0FAFF", "#F8FCFF", "#FCFEFF", "#D0F2FF", "#D8F4FF", "#E0F6FF", "#E8F8FF",
	"#F0FAFF", "#F8FCFF", "#C8F0FF", "#D0F2FF",
	// Mauve Pastels (20)
	"#E0B4D6", "#E8BCDE", "#F0C4E6", "#F8CCEE", "#FFD4F6", "#FFDCFE", "#FFE4FF", "#D8B0D4",
	"#E0B8DC", "#E8C0E4", "#F0C8EC", "#F8D0F4", "#FFD8FC", "#FFE0FF", "#D0ACD2", "#D8B4DA",
	"#E0BCE2", "#E8C4EA", "#F0CCF2", "#F8D4FA",
	// Periwinkle Pastels (20)
	"#C8C8FF", "#D0D0FF", "#D8D8FF", "#E0E0FF", "#E8E8FF", "#F0F0FF", "#F8F8FF", "#C0C0FF",
	"#C8C8FF", "#D0D0FF", "#D8D8FF", "#E0E0FF", "#E8E8FF", "#F0F0FF", "#B8B8FF", "#C0C0FF",
	"#C8C8FF", "#D0D0FF", "#D8D8FF", "#E0E0FF",
	// Seafoam Pastels (20)
	"#E0FFF0", "#E8FFF4", "#F0FFF8", "#F8FFFC", "#FCFFFE", "#FFFFFF", "#D8FFEC", "#E0FFF0",
	"#E8FFF4", "#F0FFF8", "#F8FFFC", "#FCFFFE", "#D0FFE8", "#D8FFEC", "#E0FFF0", "#E8FFF4",
	"#F0FFF8", "#F8FFFC", "#C8FFE4", "#D0FFE8",
	// Blush Pastels (20)
	"#FFE0E0", "#FFE4E4", "#FFE8E8", "#FFECEC", "#FFF0F0", "#FFF4F4", "#FFF8F8", "#FFFCFC",
	"#FFDCDC", "#FFE0E0", "#FFE4E4", "#FFE8E8", "#FFECEC", "#FFF0F0", "#FFD8D8", "#FFDCDC",
	"#FFE0E0", "#FFE4E4", "#FFE8E8", "#FFECEC",
	// Champagne Pastels (20)
	"#F7E7CE", "#F9EADD", "#FBECC5", "#FDEECD", "#FFF0D6", "#FFF2DE", "#FFF4E7", "#FFF6EF",
	"#F5E5C8", "#F7E7D0", "#F9E9D8", "#FBEBE0", "#FDEDE8", "#FFEFF0", "#F3E3C2", "#F5E5CA",
	"#F7E7D2", "#F9E9DA", "#FBEBE2", "#FDEDEA",
	// Powder Blue Pastels (20)
	"#E6F3FF", "#EBF6FF", "#F0F9FF", "#F5FCFF", "#FAFEFF", "#FFFFFF", "#E1F0FF", "#E6F3FF",
	"#EBF6FF", "#F0F9FF", "#F5FCFF", "#FAFEFF", "#DCEDFF", "#E1F0FF", "#E6F3FF", "#EBF6FF",
	"#F0F9FF", "#F5FCFF", "#D7EAFF", "#DCEDFF",
	// Lilac Pastels (20)
	"#E0D0F0", "#E6D6F6", "#ECDCFC", "#F2E2FF", "#F8E8FF", "#FEEEFF", "#FFF4FF", "#DACCEC",
	"#E0D2F2", "#E6D8F8", "#ECDEFE", "#F2E4FF", "#F8EAFF", "#FEF0FF", "#D4C8E8", "#DACIEE",
	"#E0D0F4", "#E6D6FA", "#ECDCFF", "#F2E2FF",
	// Apricot Pastels (20)
	"#FFE4B5", "#FFEAD5", "#FFF0D5", "#FFF6E5", "#FFFCF5", "#FFFFFF", "#FFDEA8", "#FFE4B8",
	"#FFEAC8", "#FFF0D8", "#FFF6E8", "#FFFCF8", "#FFD89B", "#FFDEA8", "#FFE4B5", "#FFEAC2",
	"#FFF0CF", "#FFF6DC", "#FFD28E", "#FFD89B",
	// Baby Blue Pastels (20)
	"#E0F0FF", "#E6F4FF", "#ECF8FF", "#F2FCFF", "#F8FEFF", "#FFFFFF", "#DAECFF", "#E0F0FF",
	"#E6F4FF", "#ECF8FF", "#F2FCFF", "#F8FEFF", "#D4E8FF", "#DAECFF", "#E0F0FF", "#E6F4FF",
	"#ECF8FF", "#F2FCFF", "#CEE4FF", "#D4E8FF",
	// Vanilla Pastels (20)
	"#FFF8E1", "#FFFAEA", "#FFFCF3", "#FFFEFC", "#FFFFFF", "#FFFFFF", "#FFF6D4", "#FFF8DD",
	"#FFFAE6", "#FFFCEF", "#FFFEF8", "#FFFFFF", "#FFF4C7", "#FFF6D0", "#FFF8D9", "#FFFAE2",
	"#FFFCEB", "#FFFEF4", "#FFF2BA", "#FFF4C3"
];

export const pastelColorsByType = {
	pink: pastelColors.slice(0, 20),
	blue: pastelColors.slice(20, 40),
	purple: pastelColors.slice(40, 60),
	green: pastelColors.slice(60, 80),
	red: pastelColors.slice(80, 100),
	yellow: pastelColors.slice(100, 120),
	orange: pastelColors.slice(120, 140),
	mint: pastelColors.slice(140, 160),
	coral: pastelColors.slice(160, 180),
	lavender: pastelColors.slice(180, 200),
	teal: pastelColors.slice(200, 220),
	cream: pastelColors.slice(220, 240),
	rose: pastelColors.slice(240, 260),
	sage: pastelColors.slice(260, 280),
	skyBlue: pastelColors.slice(280, 300),
	mauve: pastelColors.slice(300, 320),
	periwinkle: pastelColors.slice(320, 340),
	seafoam: pastelColors.slice(340, 360),
	blush: pastelColors.slice(360, 380),
	champagne: pastelColors.slice(380, 400),
	powderBlue: pastelColors.slice(400, 420),
	lilac: pastelColors.slice(420, 440),
	apricot: pastelColors.slice(440, 460),
	babyBlue: pastelColors.slice(460, 480),
	vanilla: pastelColors.slice(480, 500)
};

export const pastelColors_2 = [
	// Pinks & Roses
	"#FF9AA2", // Coral Blush
	"#FFB3BA", // Peach Cream
	"#FFAAA5", // Sunset Pink
	"#FF8A95", // Rose Quartz
	"#FFB5B5", // Salmon Pink
	"#F8BBD9", // Cotton Candy
	"#E8A5C4", // Dusty Pink
	"#FF91A4", // Flamingo
	"#FFB7C5", // Cherry Blossom
	"#FF7F9A", // Bubblegum
	"#FFB6C1", // Light Pink
	"#FFCCCB", // Misty Rose
	"#F5A3A3", // Rosy Brown
	"#FF69B4", // Hot Pink (pastel version)
	"#FFB3D6", // Carnation Pink

	// Purples & Lavenders
	"#C7CEEA", // Lavender Mist
	"#B5A7E6", // Periwinkle
	"#DDA0DD", // Lilac Dream
	"#C9A9DD", // Wisteria
	"#B19CD9", // Amethyst Light
	"#E6E6FA", // Lavender
	"#D8BFD8", // Thistle
	"#DA70D6", // Orchid
	"#9370DB", // Medium Purple
	"#BA55D3", // Medium Orchid
	"#DDB3FF", // Light Purple
	"#C8A2C8", // Lilac
	"#CCCCFF", // Light Slate Blue
	"#B388EB", // Bright Lilac
	"#D4A5D4", // Plum Light

	// Blues
	"#87CEEB", // Sky Pastel
	"#B0E0E6", // Powder Blue
	"#9BB5FF", // Cornflower
	"#89CFF0", // Baby Blue
	"#CCCCFF", // Periwinkle Blue
	"#ADD8E6", // Light Blue
	"#B0C4DE", // Light Steel Blue
	"#87CEFA", // Light Sky Blue
	"#E0F6FF", // Alice Blue
	"#F0F8FF", // Ghost White Blue
	"#B6D7FF", // Powder Blue Light
	"#A4D3EE", // Sky Blue Light
	"#C4E4FF", // Ice Blue
	"#7EC8E3", // Medium Sky Blue
	"#92C5F7", // Cornflower Blue

	// Greens
	"#98FB98", // Mint Fresh
	"#B2D3C2", // Sage Green
	"#93E9BE", // Seafoam
	"#AAD8A7", // Eucalyptus
	"#C1E1C1", // Pistachio
	"#90EE90", // Light Green
	"#98FB98", // Pale Green
	"#F0FFF0", // Honeydew
	"#AFEEEE", // Pale Turquoise
	"#B4D4B8", // Mint Green
	"#C7FFCB", // Light Mint
	"#A8E6A3", // Pastel Green
	"#96DED1", // Turquoise Light
	"#B5E7A0", // Tea Green
	"#CCFFCC", // Very Light Green

	// Yellows & Golds
	"#FFEB9C", // Butter Yellow
	"#FFFACD", // Lemon Chiffon
	"#F3E5AB", // Vanilla Cream
	"#FFEAA7", // Warm Yellow
	"#FFE5B4", // Peach
	"#FFFF99", // Canary Yellow
	"#FFFFCC", // Light Yellow
	"#FFF8DC", // Cornsilk
	"#FAFAD2", // Light Goldenrod
	"#F0E68C", // Khaki
	"#FFE4B5", // Moccasin
	"#FFEFD5", // Papaya Whip
	"#FFEBCD", // Blanched Almond
	"#FFE4E1", // Misty Rose
	"#FFF5EE", // Seashell

	// Oranges & Peaches
	"#FFCC99", // Peach Fuzz
	"#FBCEB1", // Apricot
	"#FFCC8F", // Cantaloupe
	"#FFD0A6", // Peach Orange
	"#FFAB91", // Light Orange
	"#FFE0B2", // Peach Cream
	"#FFDAB9", // Peach Puff
	"#FFE4B5", // Moccasin
	"#FFDEAD", // Navajo White
	"#F5DEB3", // Wheat
	"#DEB887", // Burlywood
	"#FFB347", // Light Orange
	"#FFCBA4", // Desert Sand
	"#FFDAB3", // Apricot Cream
	"#FFE5CC", // Champagne Pink

	// Reds & Corals
	"#FFA07A", // Light Salmon
	"#FA8072", // Salmon
	"#F08080", // Light Coral
	"#CD5C5C", // Indian Red
	"#FF6347", // Tomato
	"#FF7F50", // Coral
	"#FF4500", // Orange Red
	"#DC143C", // Crimson
	"#B22222", // Fire Brick
	"#A0522D", // Sienna
	"#FF69B4", // Deep Pink
	"#FF1493", // Deep Pink
	"#FF6B6B", // Light Red
	"#FF8E8E", // Pastel Red
	"#FF9F9F", // Rosy Red

	// Teals & Aquas
	"#AFEEEE", // Pale Turquoise
	"#40E0D0", // Turquoise
	"#48D1CC", // Medium Turquoise
	"#00CED1", // Dark Turquoise
	"#5F9EA0", // Cadet Blue
	"#B0E0E6", // Powder Blue
	"#AFEEEE", // Pale Turquoise
	"#E0FFFF", // Light Cyan
	"#F0FFFF", // Azure
	"#B8F2FF", // Light Aqua
	"#88E5A3", // Mint Aqua
	"#7FDBDA", // Aqua Light
	"#9FE2BF", // Sea Green Light
	"#A7FFEB", // Aquamarine Light
	"#85E3FF", // Sky Aqua

	// Neutrals & Grays
	"#F7E7CE", // Champagne
	"#F8F6F0", // Pearl White
	"#D3D3D3", // Light Gray
	"#DCDCDC", // Gainsboro
	"#F5F5F5", // White Smoke
	"#F0F0F0", // Light Gray
	"#E6E6E6", // Very Light Gray
	"#EBEBEB", // Platinum
	"#F8F8FF", // Ghost White
	"#FAFAFA", // Snow White
	"#F5F5DC", // Beige
	"#FDF5E6", // Old Lace
	"#FAF0E6", // Linen
	"#FFF8DC", // Cornsilk
	"#FFFAF0", // Floral White
];


export function shuffle<T>(array: T[]): T[] {
	const result = [...array];
	for (let i = result.length - 1; i > 0; i--) {
		const j = Math.floor(Math.random() * (i + 1));
		[result[i], result[j]] = [result[j], result[i]];
	}
	return result;
}

export function invertColor(hex: string, bw = true) {
	if (typeof hex !== 'string') {
		throw new TypeError('Expected a string for hex color');
	}

	// Remove hash if present
	if (hex.startsWith('#')) {
		hex = hex.slice(1);
	}

	// Convert 3-digit hex to 6-digit
	if (hex.length === 3) {
		hex = hex.split('').map(c => c + c).join('');
	}

	// Validate length
	if (hex.length !== 6 || !/^[0-9a-f]{6}$/i.test(hex)) {
		throw new Error('Invalid HEX color.');
	}

	// Convert to RGB
	const r = parseInt(hex.slice(0, 2), 16);
	const g = parseInt(hex.slice(2, 4), 16);
	const b = parseInt(hex.slice(4, 6), 16);

	if (bw) {
		// Calculate brightness and return black or white
		const brightness = 0.299 * r + 0.587 * g + 0.114 * b;
		return brightness > 186 ? '#000000' : '#FFFFFF';
	}

	// Invert colors and pad with zeros if needed
	const inverted = [r, g, b].map(c => (255 - c).toString(16).padStart(2, '0')).join('');
	return `#${inverted}`;
}