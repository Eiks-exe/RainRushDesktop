const SECONDS_PER_DAY = 86400;
const HOURS_PER_DAY = 24;

/**
 * Convert seconds to HH:MM:SS
 * If seconds exceeds 24 hours, hours will be greater than 24 (30:05:10)
 *
 * @param {number} seconds
 * @returns {string}
 */
const secondsToHms = (seconds: number): string => {
  const days = Math.floor(seconds / SECONDS_PER_DAY);
  const remainderSeconds = seconds % SECONDS_PER_DAY;
  const hms = new Date(remainderSeconds * 1000).toISOString().substring(11, 19);
  return hms.replace(/^(\d+)/, h => `${Number(h) + days * HOURS_PER_DAY}`.padStart(2, '0'));
};

const formatResult= (result: string): string => {
  switch (result) {
    case "StandardLoss":
      return "Loss";
    case "MainEnding":
      return "Win";
    default:
      return "Unknown";
  }
}

const formatSurvivor = (survivor: string | undefined): string => {
  if (!survivor) {
    return "Unknown";
  }
  const formattedSurvivor = survivor.split("Body")[0];
  const formattedSurvivor2= formattedSurvivor.charAt(0).toUpperCase() + formattedSurvivor.slice(1);
  switch (formattedSurvivor2) {
    case "Bandit2":
      return "Bandit";
    case "Engineer":
      return "Engineer";
    case "Huntress":
      return "Huntress";
    case "Merc":
      return "Mercenary";
    case "Miner":
      return "Miner";
    case "Rex":
      return "Rex";
    case "Loader":
      return "Loader";
    case "Artificer":
      return "Artificer";
    case "Captain":
      return "Captain";
    default:
      return formattedSurvivor;
  }
}
export {secondsToHms, formatResult, formatSurvivor};


