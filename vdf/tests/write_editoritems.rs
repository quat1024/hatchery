#![allow(dead_code)]

use std::{collections::BTreeMap, u32};

use serde::Serialize;
use vdf::named_seq_func;

#[test]
fn main() {
	let mut props: BTreeMap<String, PropertySettings> = BTreeMap::new();
	props.insert("TimerDelay".into(), PropertySettings {
		default_value: "3".into(),
		index: 1
	});
	
	props.insert("TimerSound".into(), PropertySettings {
		default_value: "0".into(),
		index: 2
	});
	
	let pedestal_button = Item {
		item_class: ItemClass::PedestalButton,
		item_type: "ITEM_BUTTON_PEDESTAL".into(),
		editor: EditorBlock {
			sub_type_property: None,
			sub_types: vec![ SubTypeBlock {
				name: "PORTAL2_PuzzleEditor_Item_pedestal_button".into(),
				model: Model {
					name: "switch.3ds".into(),
					texture: "buttonpedestal.png".into()
				},
				palette: Palette {
					tooltip: "PORTAL2_PuzzleEditor_Palette_pedestal_button".into(),
					image: "palette/pedestal_button.png".into(),
					position: "0 0 0".into()
				},
				sounds: Sounds {
					create: "P2Editor.PlaceButton".into(),
					activate: "P2Editor.ExpandButton".into(),
					deactivate: "P2Editor.CollapseButton".into(),
					delete: "P2Editor.RemoveButton".into()
				}
			}],
			movement_handle: MovementHandle::FourDirections
		},
		properties: props,
		exporting: ExportingBlock {
			instances: vec![ Instance {
				name: "instances/p2editor/pedestal_button.vmf".into(),
				entity_count: 7,
				brush_count: 1,
				brush_side_count: 6
			}],
		    target_name: "button".into(),
		    offset: "64 64 64".into(),
		    occupied_voxels: (),
		    embedded_voxels: (),
		}
	};
	
	let data = ItemData {
		items: vec![pedestal_button]
	};
	
	let serialized = vdf::ser::to_string_with_toplevel_block(&data, "ItemData").expect("could not serialize");
	println!("{}", serialized);
}

named_seq_func!(item_seq "Item");
#[derive(Serialize)]
struct ItemData {
	#[serde(flatten)]
	#[serde(serialize_with = "item_seq")]
	items: Vec<Item>
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct Item {
	item_class: ItemClass,
	#[serde(rename = "type")] //reserved word in Rust
	item_type: String,
	editor: EditorBlock,
	properties: BTreeMap<String, PropertySettings>,
	exporting: ExportingBlock
}

#[derive(Serialize)]
enum ItemClass {
	#[serde(rename = "ItemPedestalButton")]
	PedestalButton,
	#[serde(rename = "ItemButtonFloor")]
	FloorButton,
	//etc
}

named_seq_func!(subtype_seq "SubType");

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct EditorBlock {
	#[serde(skip_serializing_if = "Option::is_none")]
	sub_type_property: Option<SubTypeProperty>,
	#[serde(flatten)]
	#[serde(serialize_with = "subtype_seq")]
	sub_types: Vec<SubTypeBlock>,
	movement_handle: MovementHandle
}

#[derive(Serialize)]
enum SubTypeProperty {
	#[serde(rename = "ButtonType")]
	Button,
	#[serde(rename = "CubeType")]
	Cube,
	#[serde(rename = "HazardType")]
	Hazard,
	#[serde(rename = "BarrierType")]
	Barrier,
	#[serde(rename = "PaintType")]
	Paint
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct SubTypeBlock {
	name: String,
	model: Model,
	palette: Palette,
	sounds: Sounds
}

#[derive(Serialize)]
struct Model {
	#[serde(rename = "ModelName")]
	name: String,
	#[serde(rename = "TextureName")]
	texture: String,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct Palette {
	tooltip: String,
	image: String,
	position: String, //TODO: (u8, u8, u8)
}

#[derive(Serialize)]
struct Sounds {
	#[serde(rename = "SOUND_CREATED")]
	create: String,
	#[serde(rename = "SOUND_EDITING_ACTIVATE")]
	activate: String,
	#[serde(rename = "SOUND_EDITING_DEACTIVATE")]
	deactivate: String,
	#[serde(rename = "SOUND_DELETED")]
	delete: String
}

#[derive(Serialize)]
enum MovementHandle {
	#[serde(rename = "HANDLE_NONE")]
	None,
	#[serde(rename = "HANDLE_5_POSITIONS")]
	FivePositions,
	#[serde(rename = "HANDLE_6_POSITIONS")]
	SixPositions,
	#[serde(rename = "HANDLE_8_POSITIONS")]
	EightPositions,
	#[serde(rename = "HANDLE_4_DIRECTIONS")]
	FourDirections,
	#[serde(rename = "HANDLE_36_DIRECTIONS")]
	ThirtySixDirections,
	#[serde(rename = "HANDLE_CATAPULT")]
	Catapult
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct PropertySettings {
	default_value: String,
	index: usize
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct ExportingBlock {
	instances: Vec<Instance>,
	target_name: String,
	offset: String, //TODO vec3
	occupied_voxels: (), //TODO
	embedded_voxels: ()  //TODO
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct Instance {
	name: String,
	entity_count: u32,
	brush_count: u32,
	brush_side_count: u32
}