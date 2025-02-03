const mapBounds = {
  minLon: -169,
  maxLon: 189,
  minLat: -57.5,
  maxLat: 83.6,
};

function latToMercator(lat: number) {
  return Math.log(Math.tan((lat * Math.PI) / 180 / 2 + Math.PI / 4));
}

function convertToRatio(lat: number, lon: number) {
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
  const x = xRatio;

  const yRaw = latToMercator(lat);
  const yMin = latToMercator(mapBounds.minLat);
  const yMax = latToMercator(mapBounds.maxLat);
  const yRatio = (yRaw - yMin) / (yMax - yMin);
  const y = 1 - yRatio;

  return { x, y };
}

export function useMap() {
  return {
    convertToRatio,
  };
}
