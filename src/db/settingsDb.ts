import Database from "@tauri-apps/plugin-sql";

let db: Database | null = null;

export async function initSettingsDb() {
	if (db) return db;

	try {
		db = await Database.load("sqlite:settings.db");

		// Create settings table if it doesn't exist
		await db.execute(`
			CREATE TABLE IF NOT EXISTS settings (
				key TEXT PRIMARY KEY,
				value TEXT NOT NULL,
				updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
			)
		`);

		console.log("Settings database initialized");
		return db;
	} catch (error) {
		console.error("Failed to initialize settings database:", error);
		throw error;
	}
}

export async function getSetting(key: string): Promise<any> {
	const database = await initSettingsDb();

	try {
		const result = await database.select<{ value: string }[]>(
			"SELECT value FROM settings WHERE key = $1",
			[key]
		);

		if (result.length > 0) {
			return JSON.parse(result[0].value);
		}
		return null;
	} catch (error) {
		console.error(`Failed to get setting ${key}:`, error);
		return null;
	}
}

export async function setSetting(key: string, value: any): Promise<void> {
	const database = await initSettingsDb();

	try {
		const jsonValue = JSON.stringify(value);

		await database.execute(
			`INSERT INTO settings (key, value, updated_at) 
			 VALUES ($1, $2, CURRENT_TIMESTAMP)
			 ON CONFLICT(key) DO UPDATE SET 
			 value = $2, 
			 updated_at = CURRENT_TIMESTAMP`,
			[key, jsonValue]
		);
	} catch (error) {
		console.error(`Failed to set setting ${key}:`, error);
		throw error;
	}
}

export async function getAllSettings(): Promise<Record<string, any>> {
	const database = await initSettingsDb();

	try {
		const result = await database.select<{ key: string; value: string }[]>(
			"SELECT key, value FROM settings"
		);

		const settings: Record<string, any> = {};
		for (const row of result) {
			settings[row.key] = JSON.parse(row.value);
		}
		return settings;
	} catch (error) {
		console.error("Failed to get all settings:", error);
		return {};
	}
}

export async function deleteSetting(key: string): Promise<void> {
	const database = await initSettingsDb();

	try {
		await database.execute("DELETE FROM settings WHERE key = $1", [key]);
	} catch (error) {
		console.error(`Failed to delete setting ${key}:`, error);
		throw error;
	}
}
