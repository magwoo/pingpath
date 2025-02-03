const mapBounds = {
  minLon: -169,
  maxLon: 193,
  minLat: -60,
  maxLat: 83,
};

const mapWidth = 796;
const mapHeight = 525;

function latToMercator(lat) {
  return Math.log(Math.tan((lat * Math.PI) / 180 / 2 + Math.PI / 4));
}

function convertToPixels(lat, lon) {
  if (
    lon < mapBounds.minLon ||
    lon > mapBounds.maxLon ||
    lat < mapBounds.minLat ||
    lat > mapBounds.maxLat
  ) {
    return { x: null, y: null };
  }

  const xRatio =
    (lon - mapBounds.minLon) / (mapBounds.maxLon - mapBounds.minLon);
  const x = xRatio * mapWidth;

  const yRaw = latToMercator(lat);
  const yMin = latToMercator(mapBounds.minLat);
  const yMax = latToMercator(mapBounds.maxLat);
  const yRatio = (yRaw - yMin) / (yMax - yMin);
  const y = (1 - yRatio) * mapHeight;

  return { x, y };
}

// Тест: Гринвич (0°, 0°)
let pos1 = convertToPixels(52.090736, 5.12142); // южная америка много рек на материке справа красная
let pos2 = convertToPixels(-42.765666, 146.852519); // островок под австралией зеленый
let pos3 = convertToPixels(-12.01867, 49.253793); // верх мадагаскара синий
let pos4 = convertToPixels(37.611512, 14.157612); // италия фиол
let pos5 = convertToPixels(51.4825766, 0); // гринвич черный

let point1 = document.getElementById("point1");
point1.style.left = `${pos1.x}px`;
point1.style.top = `${pos1.y}px`;
console.log(`${pos1.x} ${pos1.y}`);
