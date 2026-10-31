// index.html tek kaynak; Tauri'ye ui/index.html olarak kopyalanır.
import { copyFileSync, mkdirSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const kok = dirname(dirname(fileURLToPath(import.meta.url)));
mkdirSync(join(kok, 'ui'), { recursive: true });
copyFileSync(join(kok, 'index.html'), join(kok, 'ui', 'index.html'));
console.log('ui/index.html güncellendi');
