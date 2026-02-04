# JSON Export Feature - Desktop Postflop

## สรุป (Summary)

เพิ่มฟีเจอร์ Export JSON ที่ให้ส่งออกข้อมูล metadata จากไฟล์ binary (.bin/.flop) เป็น JSON เพื่อดูข้อมูลพื้นฐานของเกมโดยไม่ต้องเปิดไฟล์ binary

Added JSON export feature to export game metadata from binary files for easy inspection without opening the full binary file.

---

## ข้อมูลที่อ่านจากไฟล์ Binary (Data Read from Binary File)

### Rust Backend (`GameLoadResponse`)
```rust
pub struct GameLoadResponse {
    pub memo: String,              // คำอธิบายเกม
    pub is_solved: bool,           // สถานะว่า solve แล้วหรือยัง
    pub storage_mode: String,      // "flop", "turn", หรือ "river"
    pub board: Vec<u8>,            // การ์ดบนโต๊ะ (encoded)
    pub starting_pot: i32,         // พอทเริ่มต้น
    pub effective_stack: i32,      // สแต็คที่มีผล
    pub private_cards: [Vec<u16>; 2], // การ์ดส่วนตัวของผู้เล่นทั้ง 2 คน
}
```

### TypeScript Frontend
```typescript
type GameLoadResponse = {
  memo: string;
  is_solved: boolean;
  storage_mode: "flop" | "turn" | "river";
  board: number[];
  starting_pot: number;
  effective_stack: number;
  private_cards: number[][];
}
```

### การแสดงผลบน UI (UI Display)
ข้อมูลที่โหลดจะถูกแสดงใน:
1. **NavBar.vue** - แสดง storage mode (flop/turn/river)
2. **Config Store** - อัพเดท board, starting_pot, effective_stack
3. **Store State** - อัพเดทสถานะ isFileLoaded, loadedFileMemo, isSolverFinished
4. **Results View** - นำทางไปหน้า results อัตโนมัติ

---

## ไฟล์ที่แก้ไข (Modified Files)

### 1. Frontend (TypeScript/Vue)
- **`src/components/NavBar.vue`** - เพิ่มปุ่ม "Export JSON" และฟังก์ชัน `exportJSON()`

---

## วิธีใช้งาน (How to Use)

### 1. โหลดไฟล์ Binary
- คลิกปุ่ม "Load" และเลือกไฟล์ `.flop` หรือ `.bin`
- รอให้โหลดเสร็จ

### 2. Export เป็น JSON
- คลิกปุ่ม "Export JSON" (สีน้ำเงิน) ที่จะปรากฏหลังโหลดไฟล์
- เลือกตำแหน่งที่จะบันทึกไฟล์ JSON
- ไฟล์ JSON จะถูกสร้างพร้อมข้อมูล metadata

### 3. ดูข้อมูลใน JSON
ไฟล์ JSON จะมีข้อมูลดังนี้:
```json
{
  "memo": "คำอธิบายเกม",
  "is_solved": true,
  "storage_mode": "flop",
  "board": [48, 49, 26],
  "starting_pot": 20,
  "effective_stack": 100,
  "exported_at": "2026-02-04T07:00:00.000Z",
  "note": "This is a JSON preview of the binary game file. It contains metadata only, not the full solver state."
}
```

---

## โครงสร้างไฟล์ JSON ที่ Export (Exported JSON Structure)

```json
{
  "memo": "คำอธิบายเกม",
  "is_solved": true,
  "storage_mode": "flop",
  "board": [48, 49, 26],
  "starting_pot": 20,
  "effective_stack": 100,
  "exported_at": "2026-02-04T07:00:00.000Z",
  "note": "This is a JSON preview of the binary game file. It contains metadata only, not the full solver state."
}
```

### ฟิลด์ต่างๆ (Fields)
- **memo**: คำอธิบายเกมที่บันทึกไว้
- **is_solved**: `true` ถ้า solve เสร็จแล้ว, `false` ถ้ายังไม่ได้ solve
- **storage_mode**: โหมดการเก็บข้อมูล (`"flop"`, `"turn"`, หรือ `"river"`)
- **board**: การ์ดบนโต๊ะในรูปแบบตัวเลข (encoded)
- **starting_pot**: ขนาดพอทเริ่มต้น
- **effective_stack**: ขนาดสแต็คที่มีผล
- **exported_at**: เวลาที่ export (ISO 8601 format)
- **note**: คำอธิบายเพิ่มเติม

### การ encode การ์ด (Card Encoding)
- การ์ดแต่ละใบเป็นตัวเลข u8 (0-51)
- คำนวณจาก: `rank * 4 + suit`
- Rank: 0=2, 1=3, ..., 12=A
- Suit: 0=♣, 1=♦, 2=♥, 3=♠

ตัวอย่าง:
- Ah (Ace of Hearts) = 12 * 4 + 2 = 50
- Kh (King of Hearts) = 11 * 4 + 2 = 46
- 7c (Seven of Clubs) = 5 * 4 + 0 = 20

---

## ข้อจำกัด (Limitations)

⚠️ **สำคัญ**: JSON ที่ export ออกมาเป็น **metadata เท่านั้น**
- ไม่มีข้อมูล strategy, equity, EV
- ไม่มีข้อมูล game tree
- ไม่สามารถนำไปใช้ต่อได้
- ใช้สำหรับดูข้อมูลพื้นฐานเท่านั้น

**หากต้องการบันทึกเกมเต็มรูปแบบ**: ใช้ปุ่ม "Save" เพื่อบันทึกเป็นไฟล์ `.flop`

---

## การแปลงการ์ดจากตัวเลขเป็นชื่อ (Card Decoding)

### JavaScript/TypeScript
```typescript
const ranks = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
const suits = ['♣', '♦', '♥', '♠'];

function decodeCard(cardNum: number): string {
  const rank = ranks[Math.floor(cardNum / 4)];
  const suit = suits[cardNum % 4];
  return rank + suit;
}

// ตัวอย่าง
console.log(decodeCard(48)); // "Kh"
console.log(decodeCard(49)); // "Ks"
console.log(decodeCard(26)); // "7♥"
```

### Python
```python
ranks = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A']
suits = ['♣', '♦', '♥', '♠']

def decode_card(card_num):
    rank = ranks[card_num // 4]
    suit = suits[card_num % 4]
    return rank + suit

# ตัวอย่าง
print(decode_card(48))  # "Kh"
print(decode_card(49))  # "Ks"
print(decode_card(26))  # "7♥"
```

---

## Use Cases

### 1. Quick Inspection
ดูข้อมูลพื้นฐานของไฟล์ solver โดยไม่ต้องโหลดเข้า UI

### 2. Batch Processing
ใช้ script อ่านข้อมูลจากหลายไฟล์พร้อมกัน

### 3. Documentation
บันทึกข้อมูล configuration ของเกมต่างๆ

### 4. Debugging
ตรวจสอบว่าไฟล์มีข้อมูลถูกต้องหรือไม่

---

## Example: Batch Export Script

```bash
#!/bin/bash
# Export all .flop files to JSON

for file in *.flop; do
  echo "Processing $file..."
  # Load file in app and click Export JSON
  # Or create a CLI tool to do this
done
```

---

## Future Enhancements

- [ ] เพิ่ม CLI tool สำหรับ export JSON โดยไม่ต้องเปิด UI
- [ ] เพิ่มข้อมูล tree configuration ใน JSON
- [ ] เพิ่มข้อมูล ranges ใน JSON
- [ ] Support import JSON กลับเป็น binary (ถ้าเป็นไปได้)
- [ ] เพิ่ม validation สำหรับ JSON format

---

## Technical Details

### Flow
1. User โหลดไฟล์ binary → `game_load_file()`
2. Game state ถูก initialize และเก็บใน Rust
3. User คลิก "Export JSON" → `exportJSON()`
4. Frontend เรียก `gameInfo()` เพื่อดึงข้อมูล metadata
5. สร้าง JSON object และบันทึกเป็นไฟล์

### API Calls
```typescript
// Get game info (metadata only)
const info = await gameInfo();

// Create JSON
const exportData = {
  memo: store.loadedFileMemo,
  is_solved: info.is_solved,
  storage_mode: info.storage_mode,
  board: info.board,
  starting_pot: info.starting_pot,
  effective_stack: info.effective_stack,
  exported_at: new Date().toISOString(),
  note: "Metadata only..."
};

// Save to file
await writeTextFile(path, JSON.stringify(exportData, null, 2));
```

---

## Troubleshooting

### ปุ่ม Export JSON ไม่แสดง
- ตรวจสอบว่าโหลดไฟล์แล้วหรือยัง
- `store.isFileLoaded` ต้องเป็น `true`

### Export ไม่สำเร็จ
- ตรวจสอบ permissions ของโฟลเดอร์ที่จะบันทึก
- ตรวจสอบว่ามี disk space เพียงพอ

### ข้อมูลใน JSON ไม่ถูกต้อง
- ตรวจสอบว่าไฟล์ binary ที่โหลดมาถูกต้อง
- ลองโหลดไฟล์ใหม่อีกครั้ง
