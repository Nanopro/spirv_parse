#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
use serde::{Deserialize, Serialize};
pub const MAGIC: u64 = 0x07230203;
pub const VERSION: (u8, u8) = (1u8, 5u8);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ImageOperands(pub u32);
impl ImageOperands {
    pub const None: Self = ImageOperands(0u32);
    pub const Bias: Self = ImageOperands(1u32);
    pub const Lod: Self = ImageOperands(2u32);
    pub const Grad: Self = ImageOperands(4u32);
    pub const ConstOffset: Self = ImageOperands(8u32);
    pub const Offset: Self = ImageOperands(16u32);
    pub const ConstOffsets: Self = ImageOperands(32u32);
    pub const Sample: Self = ImageOperands(64u32);
    pub const MinLod: Self = ImageOperands(128u32);
    pub const MakeTexelAvailable: Self = ImageOperands(256u32);
    pub const MakeTexelAvailableKHR: Self = ImageOperands(256u32);
    pub const MakeTexelVisible: Self = ImageOperands(512u32);
    pub const MakeTexelVisibleKHR: Self = ImageOperands(512u32);
    pub const NonPrivateTexel: Self = ImageOperands(1024u32);
    pub const NonPrivateTexelKHR: Self = ImageOperands(1024u32);
    pub const VolatileTexel: Self = ImageOperands(2048u32);
    pub const VolatileTexelKHR: Self = ImageOperands(2048u32);
    pub const SignExtend: Self = ImageOperands(4096u32);
    pub const ZeroExtend: Self = ImageOperands(8192u32);
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "None" => Some(ImageOperands::None),
            "Bias" => Some(ImageOperands::Bias),
            "Lod" => Some(ImageOperands::Lod),
            "Grad" => Some(ImageOperands::Grad),
            "ConstOffset" => Some(ImageOperands::ConstOffset),
            "Offset" => Some(ImageOperands::Offset),
            "ConstOffsets" => Some(ImageOperands::ConstOffsets),
            "Sample" => Some(ImageOperands::Sample),
            "MinLod" => Some(ImageOperands::MinLod),
            "MakeTexelAvailable" => Some(ImageOperands::MakeTexelAvailable),
            "MakeTexelAvailableKHR" => Some(ImageOperands::MakeTexelAvailableKHR),
            "MakeTexelVisible" => Some(ImageOperands::MakeTexelVisible),
            "MakeTexelVisibleKHR" => Some(ImageOperands::MakeTexelVisibleKHR),
            "NonPrivateTexel" => Some(ImageOperands::NonPrivateTexel),
            "NonPrivateTexelKHR" => Some(ImageOperands::NonPrivateTexelKHR),
            "VolatileTexel" => Some(ImageOperands::VolatileTexel),
            "VolatileTexelKHR" => Some(ImageOperands::VolatileTexelKHR),
            "SignExtend" => Some(ImageOperands::SignExtend),
            "ZeroExtend" => Some(ImageOperands::ZeroExtend),
            _ => None,
        }
    }
}
impl Default for ImageOperands {
    fn default() -> ImageOperands {
        ImageOperands(0)
    }
}
impl ImageOperands {
    #[inline]
    pub const fn empty() -> ImageOperands {
        ImageOperands(0)
    }
    #[inline]
    pub const fn all() -> ImageOperands {
        ImageOperands(8192u32 as u32)
    }
    #[inline]
    pub fn from_raw(data: &[u32]) -> (Self, &[u32]) {
        (ImageOperands(data[0]), &data[1..])
    }
    #[inline]
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    #[inline]
    pub fn is_empty(self) -> bool {
        self == ImageOperands::empty()
    }
    #[inline]
    pub fn is_all(self) -> bool {
        self & ImageOperands::all() == ImageOperands::all()
    }
    #[inline]
    pub fn intersects(self, other: ImageOperands) -> bool {
        self & other != ImageOperands::empty()
    }
    #[doc = r" Returns whether `other` is a subset of `self`"]
    #[inline]
    pub fn contains(self, other: ImageOperands) -> bool {
        self & other == other
    }
}
impl ::std::ops::BitOr for ImageOperands {
    type Output = ImageOperands;
    #[inline]
    fn bitor(self, rhs: ImageOperands) -> ImageOperands {
        ImageOperands(self.0 | rhs.0)
    }
}
impl ::std::ops::BitOrAssign for ImageOperands {
    #[inline]
    fn bitor_assign(&mut self, rhs: ImageOperands) {
        *self = *self | rhs
    }
}
impl ::std::ops::BitAnd for ImageOperands {
    type Output = ImageOperands;
    #[inline]
    fn bitand(self, rhs: ImageOperands) -> ImageOperands {
        ImageOperands(self.0 & rhs.0)
    }
}
impl ::std::ops::BitAndAssign for ImageOperands {
    #[inline]
    fn bitand_assign(&mut self, rhs: ImageOperands) {
        *self = *self & rhs
    }
}
impl ::std::ops::BitXor for ImageOperands {
    type Output = ImageOperands;
    #[inline]
    fn bitxor(self, rhs: ImageOperands) -> ImageOperands {
        ImageOperands(self.0 ^ rhs.0)
    }
}
impl ::std::ops::BitXorAssign for ImageOperands {
    #[inline]
    fn bitxor_assign(&mut self, rhs: ImageOperands) {
        *self = *self ^ rhs
    }
}
impl ::std::ops::Sub for ImageOperands {
    type Output = ImageOperands;
    #[inline]
    fn sub(self, rhs: ImageOperands) -> ImageOperands {
        self & !rhs
    }
}
impl ::std::ops::SubAssign for ImageOperands {
    #[inline]
    fn sub_assign(&mut self, rhs: ImageOperands) {
        *self = *self - rhs
    }
}
impl ::std::ops::Not for ImageOperands {
    type Output = ImageOperands;
    #[inline]
    fn not(self) -> ImageOperands {
        self ^ ImageOperands::all()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct FPFastMathMode(pub u32);
impl FPFastMathMode {
    pub const None: Self = FPFastMathMode(0u32);
    pub const NotNaN: Self = FPFastMathMode(1u32);
    pub const NotInf: Self = FPFastMathMode(2u32);
    pub const NSZ: Self = FPFastMathMode(4u32);
    pub const AllowRecip: Self = FPFastMathMode(8u32);
    pub const Fast: Self = FPFastMathMode(16u32);
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "None" => Some(FPFastMathMode::None),
            "NotNaN" => Some(FPFastMathMode::NotNaN),
            "NotInf" => Some(FPFastMathMode::NotInf),
            "NSZ" => Some(FPFastMathMode::NSZ),
            "AllowRecip" => Some(FPFastMathMode::AllowRecip),
            "Fast" => Some(FPFastMathMode::Fast),
            _ => None,
        }
    }
}
impl Default for FPFastMathMode {
    fn default() -> FPFastMathMode {
        FPFastMathMode(0)
    }
}
impl FPFastMathMode {
    #[inline]
    pub const fn empty() -> FPFastMathMode {
        FPFastMathMode(0)
    }
    #[inline]
    pub const fn all() -> FPFastMathMode {
        FPFastMathMode(16u32 as u32)
    }
    #[inline]
    pub fn from_raw(data: &[u32]) -> (Self, &[u32]) {
        (FPFastMathMode(data[0]), &data[1..])
    }
    #[inline]
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    #[inline]
    pub fn is_empty(self) -> bool {
        self == FPFastMathMode::empty()
    }
    #[inline]
    pub fn is_all(self) -> bool {
        self & FPFastMathMode::all() == FPFastMathMode::all()
    }
    #[inline]
    pub fn intersects(self, other: FPFastMathMode) -> bool {
        self & other != FPFastMathMode::empty()
    }
    #[doc = r" Returns whether `other` is a subset of `self`"]
    #[inline]
    pub fn contains(self, other: FPFastMathMode) -> bool {
        self & other == other
    }
}
impl ::std::ops::BitOr for FPFastMathMode {
    type Output = FPFastMathMode;
    #[inline]
    fn bitor(self, rhs: FPFastMathMode) -> FPFastMathMode {
        FPFastMathMode(self.0 | rhs.0)
    }
}
impl ::std::ops::BitOrAssign for FPFastMathMode {
    #[inline]
    fn bitor_assign(&mut self, rhs: FPFastMathMode) {
        *self = *self | rhs
    }
}
impl ::std::ops::BitAnd for FPFastMathMode {
    type Output = FPFastMathMode;
    #[inline]
    fn bitand(self, rhs: FPFastMathMode) -> FPFastMathMode {
        FPFastMathMode(self.0 & rhs.0)
    }
}
impl ::std::ops::BitAndAssign for FPFastMathMode {
    #[inline]
    fn bitand_assign(&mut self, rhs: FPFastMathMode) {
        *self = *self & rhs
    }
}
impl ::std::ops::BitXor for FPFastMathMode {
    type Output = FPFastMathMode;
    #[inline]
    fn bitxor(self, rhs: FPFastMathMode) -> FPFastMathMode {
        FPFastMathMode(self.0 ^ rhs.0)
    }
}
impl ::std::ops::BitXorAssign for FPFastMathMode {
    #[inline]
    fn bitxor_assign(&mut self, rhs: FPFastMathMode) {
        *self = *self ^ rhs
    }
}
impl ::std::ops::Sub for FPFastMathMode {
    type Output = FPFastMathMode;
    #[inline]
    fn sub(self, rhs: FPFastMathMode) -> FPFastMathMode {
        self & !rhs
    }
}
impl ::std::ops::SubAssign for FPFastMathMode {
    #[inline]
    fn sub_assign(&mut self, rhs: FPFastMathMode) {
        *self = *self - rhs
    }
}
impl ::std::ops::Not for FPFastMathMode {
    type Output = FPFastMathMode;
    #[inline]
    fn not(self) -> FPFastMathMode {
        self ^ FPFastMathMode::all()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SelectionControl(pub u32);
impl SelectionControl {
    pub const None: Self = SelectionControl(0u32);
    pub const Flatten: Self = SelectionControl(1u32);
    pub const DontFlatten: Self = SelectionControl(2u32);
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "None" => Some(SelectionControl::None),
            "Flatten" => Some(SelectionControl::Flatten),
            "DontFlatten" => Some(SelectionControl::DontFlatten),
            _ => None,
        }
    }
}
impl Default for SelectionControl {
    fn default() -> SelectionControl {
        SelectionControl(0)
    }
}
impl SelectionControl {
    #[inline]
    pub const fn empty() -> SelectionControl {
        SelectionControl(0)
    }
    #[inline]
    pub const fn all() -> SelectionControl {
        SelectionControl(2u32 as u32)
    }
    #[inline]
    pub fn from_raw(data: &[u32]) -> (Self, &[u32]) {
        (SelectionControl(data[0]), &data[1..])
    }
    #[inline]
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    #[inline]
    pub fn is_empty(self) -> bool {
        self == SelectionControl::empty()
    }
    #[inline]
    pub fn is_all(self) -> bool {
        self & SelectionControl::all() == SelectionControl::all()
    }
    #[inline]
    pub fn intersects(self, other: SelectionControl) -> bool {
        self & other != SelectionControl::empty()
    }
    #[doc = r" Returns whether `other` is a subset of `self`"]
    #[inline]
    pub fn contains(self, other: SelectionControl) -> bool {
        self & other == other
    }
}
impl ::std::ops::BitOr for SelectionControl {
    type Output = SelectionControl;
    #[inline]
    fn bitor(self, rhs: SelectionControl) -> SelectionControl {
        SelectionControl(self.0 | rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SelectionControl {
    #[inline]
    fn bitor_assign(&mut self, rhs: SelectionControl) {
        *self = *self | rhs
    }
}
impl ::std::ops::BitAnd for SelectionControl {
    type Output = SelectionControl;
    #[inline]
    fn bitand(self, rhs: SelectionControl) -> SelectionControl {
        SelectionControl(self.0 & rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SelectionControl {
    #[inline]
    fn bitand_assign(&mut self, rhs: SelectionControl) {
        *self = *self & rhs
    }
}
impl ::std::ops::BitXor for SelectionControl {
    type Output = SelectionControl;
    #[inline]
    fn bitxor(self, rhs: SelectionControl) -> SelectionControl {
        SelectionControl(self.0 ^ rhs.0)
    }
}
impl ::std::ops::BitXorAssign for SelectionControl {
    #[inline]
    fn bitxor_assign(&mut self, rhs: SelectionControl) {
        *self = *self ^ rhs
    }
}
impl ::std::ops::Sub for SelectionControl {
    type Output = SelectionControl;
    #[inline]
    fn sub(self, rhs: SelectionControl) -> SelectionControl {
        self & !rhs
    }
}
impl ::std::ops::SubAssign for SelectionControl {
    #[inline]
    fn sub_assign(&mut self, rhs: SelectionControl) {
        *self = *self - rhs
    }
}
impl ::std::ops::Not for SelectionControl {
    type Output = SelectionControl;
    #[inline]
    fn not(self) -> SelectionControl {
        self ^ SelectionControl::all()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct LoopControl(pub u32);
impl LoopControl {
    pub const None: Self = LoopControl(0u32);
    pub const Unroll: Self = LoopControl(1u32);
    pub const DontUnroll: Self = LoopControl(2u32);
    pub const DependencyInfinite: Self = LoopControl(4u32);
    pub const DependencyLength: Self = LoopControl(8u32);
    pub const MinIterations: Self = LoopControl(16u32);
    pub const MaxIterations: Self = LoopControl(32u32);
    pub const IterationMultiple: Self = LoopControl(64u32);
    pub const PeelCount: Self = LoopControl(128u32);
    pub const PartialCount: Self = LoopControl(256u32);
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "None" => Some(LoopControl::None),
            "Unroll" => Some(LoopControl::Unroll),
            "DontUnroll" => Some(LoopControl::DontUnroll),
            "DependencyInfinite" => Some(LoopControl::DependencyInfinite),
            "DependencyLength" => Some(LoopControl::DependencyLength),
            "MinIterations" => Some(LoopControl::MinIterations),
            "MaxIterations" => Some(LoopControl::MaxIterations),
            "IterationMultiple" => Some(LoopControl::IterationMultiple),
            "PeelCount" => Some(LoopControl::PeelCount),
            "PartialCount" => Some(LoopControl::PartialCount),
            _ => None,
        }
    }
}
impl Default for LoopControl {
    fn default() -> LoopControl {
        LoopControl(0)
    }
}
impl LoopControl {
    #[inline]
    pub const fn empty() -> LoopControl {
        LoopControl(0)
    }
    #[inline]
    pub const fn all() -> LoopControl {
        LoopControl(256u32 as u32)
    }
    #[inline]
    pub fn from_raw(data: &[u32]) -> (Self, &[u32]) {
        (LoopControl(data[0]), &data[1..])
    }
    #[inline]
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    #[inline]
    pub fn is_empty(self) -> bool {
        self == LoopControl::empty()
    }
    #[inline]
    pub fn is_all(self) -> bool {
        self & LoopControl::all() == LoopControl::all()
    }
    #[inline]
    pub fn intersects(self, other: LoopControl) -> bool {
        self & other != LoopControl::empty()
    }
    #[doc = r" Returns whether `other` is a subset of `self`"]
    #[inline]
    pub fn contains(self, other: LoopControl) -> bool {
        self & other == other
    }
}
impl ::std::ops::BitOr for LoopControl {
    type Output = LoopControl;
    #[inline]
    fn bitor(self, rhs: LoopControl) -> LoopControl {
        LoopControl(self.0 | rhs.0)
    }
}
impl ::std::ops::BitOrAssign for LoopControl {
    #[inline]
    fn bitor_assign(&mut self, rhs: LoopControl) {
        *self = *self | rhs
    }
}
impl ::std::ops::BitAnd for LoopControl {
    type Output = LoopControl;
    #[inline]
    fn bitand(self, rhs: LoopControl) -> LoopControl {
        LoopControl(self.0 & rhs.0)
    }
}
impl ::std::ops::BitAndAssign for LoopControl {
    #[inline]
    fn bitand_assign(&mut self, rhs: LoopControl) {
        *self = *self & rhs
    }
}
impl ::std::ops::BitXor for LoopControl {
    type Output = LoopControl;
    #[inline]
    fn bitxor(self, rhs: LoopControl) -> LoopControl {
        LoopControl(self.0 ^ rhs.0)
    }
}
impl ::std::ops::BitXorAssign for LoopControl {
    #[inline]
    fn bitxor_assign(&mut self, rhs: LoopControl) {
        *self = *self ^ rhs
    }
}
impl ::std::ops::Sub for LoopControl {
    type Output = LoopControl;
    #[inline]
    fn sub(self, rhs: LoopControl) -> LoopControl {
        self & !rhs
    }
}
impl ::std::ops::SubAssign for LoopControl {
    #[inline]
    fn sub_assign(&mut self, rhs: LoopControl) {
        *self = *self - rhs
    }
}
impl ::std::ops::Not for LoopControl {
    type Output = LoopControl;
    #[inline]
    fn not(self) -> LoopControl {
        self ^ LoopControl::all()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct FunctionControl(pub u32);
impl FunctionControl {
    pub const None: Self = FunctionControl(0u32);
    pub const Inline: Self = FunctionControl(1u32);
    pub const DontInline: Self = FunctionControl(2u32);
    pub const Pure: Self = FunctionControl(4u32);
    pub const Const: Self = FunctionControl(8u32);
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "None" => Some(FunctionControl::None),
            "Inline" => Some(FunctionControl::Inline),
            "DontInline" => Some(FunctionControl::DontInline),
            "Pure" => Some(FunctionControl::Pure),
            "Const" => Some(FunctionControl::Const),
            _ => None,
        }
    }
}
impl Default for FunctionControl {
    fn default() -> FunctionControl {
        FunctionControl(0)
    }
}
impl FunctionControl {
    #[inline]
    pub const fn empty() -> FunctionControl {
        FunctionControl(0)
    }
    #[inline]
    pub const fn all() -> FunctionControl {
        FunctionControl(8u32 as u32)
    }
    #[inline]
    pub fn from_raw(data: &[u32]) -> (Self, &[u32]) {
        (FunctionControl(data[0]), &data[1..])
    }
    #[inline]
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    #[inline]
    pub fn is_empty(self) -> bool {
        self == FunctionControl::empty()
    }
    #[inline]
    pub fn is_all(self) -> bool {
        self & FunctionControl::all() == FunctionControl::all()
    }
    #[inline]
    pub fn intersects(self, other: FunctionControl) -> bool {
        self & other != FunctionControl::empty()
    }
    #[doc = r" Returns whether `other` is a subset of `self`"]
    #[inline]
    pub fn contains(self, other: FunctionControl) -> bool {
        self & other == other
    }
}
impl ::std::ops::BitOr for FunctionControl {
    type Output = FunctionControl;
    #[inline]
    fn bitor(self, rhs: FunctionControl) -> FunctionControl {
        FunctionControl(self.0 | rhs.0)
    }
}
impl ::std::ops::BitOrAssign for FunctionControl {
    #[inline]
    fn bitor_assign(&mut self, rhs: FunctionControl) {
        *self = *self | rhs
    }
}
impl ::std::ops::BitAnd for FunctionControl {
    type Output = FunctionControl;
    #[inline]
    fn bitand(self, rhs: FunctionControl) -> FunctionControl {
        FunctionControl(self.0 & rhs.0)
    }
}
impl ::std::ops::BitAndAssign for FunctionControl {
    #[inline]
    fn bitand_assign(&mut self, rhs: FunctionControl) {
        *self = *self & rhs
    }
}
impl ::std::ops::BitXor for FunctionControl {
    type Output = FunctionControl;
    #[inline]
    fn bitxor(self, rhs: FunctionControl) -> FunctionControl {
        FunctionControl(self.0 ^ rhs.0)
    }
}
impl ::std::ops::BitXorAssign for FunctionControl {
    #[inline]
    fn bitxor_assign(&mut self, rhs: FunctionControl) {
        *self = *self ^ rhs
    }
}
impl ::std::ops::Sub for FunctionControl {
    type Output = FunctionControl;
    #[inline]
    fn sub(self, rhs: FunctionControl) -> FunctionControl {
        self & !rhs
    }
}
impl ::std::ops::SubAssign for FunctionControl {
    #[inline]
    fn sub_assign(&mut self, rhs: FunctionControl) {
        *self = *self - rhs
    }
}
impl ::std::ops::Not for FunctionControl {
    type Output = FunctionControl;
    #[inline]
    fn not(self) -> FunctionControl {
        self ^ FunctionControl::all()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct MemorySemantics(pub u32);
impl MemorySemantics {
    pub const Relaxed: Self = MemorySemantics(0u32);
    pub const None: Self = MemorySemantics(0u32);
    pub const Acquire: Self = MemorySemantics(2u32);
    pub const Release: Self = MemorySemantics(4u32);
    pub const AcquireRelease: Self = MemorySemantics(8u32);
    pub const SequentiallyConsistent: Self = MemorySemantics(16u32);
    pub const UniformMemory: Self = MemorySemantics(64u32);
    pub const SubgroupMemory: Self = MemorySemantics(128u32);
    pub const WorkgroupMemory: Self = MemorySemantics(256u32);
    pub const CrossWorkgroupMemory: Self = MemorySemantics(512u32);
    pub const AtomicCounterMemory: Self = MemorySemantics(1024u32);
    pub const ImageMemory: Self = MemorySemantics(2048u32);
    pub const OutputMemory: Self = MemorySemantics(4096u32);
    pub const OutputMemoryKHR: Self = MemorySemantics(4096u32);
    pub const MakeAvailable: Self = MemorySemantics(8192u32);
    pub const MakeAvailableKHR: Self = MemorySemantics(8192u32);
    pub const MakeVisible: Self = MemorySemantics(16384u32);
    pub const MakeVisibleKHR: Self = MemorySemantics(16384u32);
    pub const Volatile: Self = MemorySemantics(32768u32);
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "Relaxed" => Some(MemorySemantics::Relaxed),
            "None" => Some(MemorySemantics::None),
            "Acquire" => Some(MemorySemantics::Acquire),
            "Release" => Some(MemorySemantics::Release),
            "AcquireRelease" => Some(MemorySemantics::AcquireRelease),
            "SequentiallyConsistent" => Some(MemorySemantics::SequentiallyConsistent),
            "UniformMemory" => Some(MemorySemantics::UniformMemory),
            "SubgroupMemory" => Some(MemorySemantics::SubgroupMemory),
            "WorkgroupMemory" => Some(MemorySemantics::WorkgroupMemory),
            "CrossWorkgroupMemory" => Some(MemorySemantics::CrossWorkgroupMemory),
            "AtomicCounterMemory" => Some(MemorySemantics::AtomicCounterMemory),
            "ImageMemory" => Some(MemorySemantics::ImageMemory),
            "OutputMemory" => Some(MemorySemantics::OutputMemory),
            "OutputMemoryKHR" => Some(MemorySemantics::OutputMemoryKHR),
            "MakeAvailable" => Some(MemorySemantics::MakeAvailable),
            "MakeAvailableKHR" => Some(MemorySemantics::MakeAvailableKHR),
            "MakeVisible" => Some(MemorySemantics::MakeVisible),
            "MakeVisibleKHR" => Some(MemorySemantics::MakeVisibleKHR),
            "Volatile" => Some(MemorySemantics::Volatile),
            _ => None,
        }
    }
}
impl Default for MemorySemantics {
    fn default() -> MemorySemantics {
        MemorySemantics(0)
    }
}
impl MemorySemantics {
    #[inline]
    pub const fn empty() -> MemorySemantics {
        MemorySemantics(0)
    }
    #[inline]
    pub const fn all() -> MemorySemantics {
        MemorySemantics(32768u32 as u32)
    }
    #[inline]
    pub fn from_raw(data: &[u32]) -> (Self, &[u32]) {
        (MemorySemantics(data[0]), &data[1..])
    }
    #[inline]
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    #[inline]
    pub fn is_empty(self) -> bool {
        self == MemorySemantics::empty()
    }
    #[inline]
    pub fn is_all(self) -> bool {
        self & MemorySemantics::all() == MemorySemantics::all()
    }
    #[inline]
    pub fn intersects(self, other: MemorySemantics) -> bool {
        self & other != MemorySemantics::empty()
    }
    #[doc = r" Returns whether `other` is a subset of `self`"]
    #[inline]
    pub fn contains(self, other: MemorySemantics) -> bool {
        self & other == other
    }
}
impl ::std::ops::BitOr for MemorySemantics {
    type Output = MemorySemantics;
    #[inline]
    fn bitor(self, rhs: MemorySemantics) -> MemorySemantics {
        MemorySemantics(self.0 | rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MemorySemantics {
    #[inline]
    fn bitor_assign(&mut self, rhs: MemorySemantics) {
        *self = *self | rhs
    }
}
impl ::std::ops::BitAnd for MemorySemantics {
    type Output = MemorySemantics;
    #[inline]
    fn bitand(self, rhs: MemorySemantics) -> MemorySemantics {
        MemorySemantics(self.0 & rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MemorySemantics {
    #[inline]
    fn bitand_assign(&mut self, rhs: MemorySemantics) {
        *self = *self & rhs
    }
}
impl ::std::ops::BitXor for MemorySemantics {
    type Output = MemorySemantics;
    #[inline]
    fn bitxor(self, rhs: MemorySemantics) -> MemorySemantics {
        MemorySemantics(self.0 ^ rhs.0)
    }
}
impl ::std::ops::BitXorAssign for MemorySemantics {
    #[inline]
    fn bitxor_assign(&mut self, rhs: MemorySemantics) {
        *self = *self ^ rhs
    }
}
impl ::std::ops::Sub for MemorySemantics {
    type Output = MemorySemantics;
    #[inline]
    fn sub(self, rhs: MemorySemantics) -> MemorySemantics {
        self & !rhs
    }
}
impl ::std::ops::SubAssign for MemorySemantics {
    #[inline]
    fn sub_assign(&mut self, rhs: MemorySemantics) {
        *self = *self - rhs
    }
}
impl ::std::ops::Not for MemorySemantics {
    type Output = MemorySemantics;
    #[inline]
    fn not(self) -> MemorySemantics {
        self ^ MemorySemantics::all()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct MemoryAccess(pub u32);
impl MemoryAccess {
    pub const None: Self = MemoryAccess(0u32);
    pub const Volatile: Self = MemoryAccess(1u32);
    pub const Aligned: Self = MemoryAccess(2u32);
    pub const Nontemporal: Self = MemoryAccess(4u32);
    pub const MakePointerAvailable: Self = MemoryAccess(8u32);
    pub const MakePointerAvailableKHR: Self = MemoryAccess(8u32);
    pub const MakePointerVisible: Self = MemoryAccess(16u32);
    pub const MakePointerVisibleKHR: Self = MemoryAccess(16u32);
    pub const NonPrivatePointer: Self = MemoryAccess(32u32);
    pub const NonPrivatePointerKHR: Self = MemoryAccess(32u32);
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "None" => Some(MemoryAccess::None),
            "Volatile" => Some(MemoryAccess::Volatile),
            "Aligned" => Some(MemoryAccess::Aligned),
            "Nontemporal" => Some(MemoryAccess::Nontemporal),
            "MakePointerAvailable" => Some(MemoryAccess::MakePointerAvailable),
            "MakePointerAvailableKHR" => Some(MemoryAccess::MakePointerAvailableKHR),
            "MakePointerVisible" => Some(MemoryAccess::MakePointerVisible),
            "MakePointerVisibleKHR" => Some(MemoryAccess::MakePointerVisibleKHR),
            "NonPrivatePointer" => Some(MemoryAccess::NonPrivatePointer),
            "NonPrivatePointerKHR" => Some(MemoryAccess::NonPrivatePointerKHR),
            _ => None,
        }
    }
}
impl Default for MemoryAccess {
    fn default() -> MemoryAccess {
        MemoryAccess(0)
    }
}
impl MemoryAccess {
    #[inline]
    pub const fn empty() -> MemoryAccess {
        MemoryAccess(0)
    }
    #[inline]
    pub const fn all() -> MemoryAccess {
        MemoryAccess(32u32 as u32)
    }
    #[inline]
    pub fn from_raw(data: &[u32]) -> (Self, &[u32]) {
        (MemoryAccess(data[0]), &data[1..])
    }
    #[inline]
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    #[inline]
    pub fn is_empty(self) -> bool {
        self == MemoryAccess::empty()
    }
    #[inline]
    pub fn is_all(self) -> bool {
        self & MemoryAccess::all() == MemoryAccess::all()
    }
    #[inline]
    pub fn intersects(self, other: MemoryAccess) -> bool {
        self & other != MemoryAccess::empty()
    }
    #[doc = r" Returns whether `other` is a subset of `self`"]
    #[inline]
    pub fn contains(self, other: MemoryAccess) -> bool {
        self & other == other
    }
}
impl ::std::ops::BitOr for MemoryAccess {
    type Output = MemoryAccess;
    #[inline]
    fn bitor(self, rhs: MemoryAccess) -> MemoryAccess {
        MemoryAccess(self.0 | rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MemoryAccess {
    #[inline]
    fn bitor_assign(&mut self, rhs: MemoryAccess) {
        *self = *self | rhs
    }
}
impl ::std::ops::BitAnd for MemoryAccess {
    type Output = MemoryAccess;
    #[inline]
    fn bitand(self, rhs: MemoryAccess) -> MemoryAccess {
        MemoryAccess(self.0 & rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MemoryAccess {
    #[inline]
    fn bitand_assign(&mut self, rhs: MemoryAccess) {
        *self = *self & rhs
    }
}
impl ::std::ops::BitXor for MemoryAccess {
    type Output = MemoryAccess;
    #[inline]
    fn bitxor(self, rhs: MemoryAccess) -> MemoryAccess {
        MemoryAccess(self.0 ^ rhs.0)
    }
}
impl ::std::ops::BitXorAssign for MemoryAccess {
    #[inline]
    fn bitxor_assign(&mut self, rhs: MemoryAccess) {
        *self = *self ^ rhs
    }
}
impl ::std::ops::Sub for MemoryAccess {
    type Output = MemoryAccess;
    #[inline]
    fn sub(self, rhs: MemoryAccess) -> MemoryAccess {
        self & !rhs
    }
}
impl ::std::ops::SubAssign for MemoryAccess {
    #[inline]
    fn sub_assign(&mut self, rhs: MemoryAccess) {
        *self = *self - rhs
    }
}
impl ::std::ops::Not for MemoryAccess {
    type Output = MemoryAccess;
    #[inline]
    fn not(self) -> MemoryAccess {
        self ^ MemoryAccess::all()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct KernelProfilingInfo(pub u32);
impl KernelProfilingInfo {
    pub const None: Self = KernelProfilingInfo(0u32);
    pub const CmdExecTime: Self = KernelProfilingInfo(1u32);
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "None" => Some(KernelProfilingInfo::None),
            "CmdExecTime" => Some(KernelProfilingInfo::CmdExecTime),
            _ => None,
        }
    }
}
impl Default for KernelProfilingInfo {
    fn default() -> KernelProfilingInfo {
        KernelProfilingInfo(0)
    }
}
impl KernelProfilingInfo {
    #[inline]
    pub const fn empty() -> KernelProfilingInfo {
        KernelProfilingInfo(0)
    }
    #[inline]
    pub const fn all() -> KernelProfilingInfo {
        KernelProfilingInfo(1u32 as u32)
    }
    #[inline]
    pub fn from_raw(data: &[u32]) -> (Self, &[u32]) {
        (KernelProfilingInfo(data[0]), &data[1..])
    }
    #[inline]
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    #[inline]
    pub fn is_empty(self) -> bool {
        self == KernelProfilingInfo::empty()
    }
    #[inline]
    pub fn is_all(self) -> bool {
        self & KernelProfilingInfo::all() == KernelProfilingInfo::all()
    }
    #[inline]
    pub fn intersects(self, other: KernelProfilingInfo) -> bool {
        self & other != KernelProfilingInfo::empty()
    }
    #[doc = r" Returns whether `other` is a subset of `self`"]
    #[inline]
    pub fn contains(self, other: KernelProfilingInfo) -> bool {
        self & other == other
    }
}
impl ::std::ops::BitOr for KernelProfilingInfo {
    type Output = KernelProfilingInfo;
    #[inline]
    fn bitor(self, rhs: KernelProfilingInfo) -> KernelProfilingInfo {
        KernelProfilingInfo(self.0 | rhs.0)
    }
}
impl ::std::ops::BitOrAssign for KernelProfilingInfo {
    #[inline]
    fn bitor_assign(&mut self, rhs: KernelProfilingInfo) {
        *self = *self | rhs
    }
}
impl ::std::ops::BitAnd for KernelProfilingInfo {
    type Output = KernelProfilingInfo;
    #[inline]
    fn bitand(self, rhs: KernelProfilingInfo) -> KernelProfilingInfo {
        KernelProfilingInfo(self.0 & rhs.0)
    }
}
impl ::std::ops::BitAndAssign for KernelProfilingInfo {
    #[inline]
    fn bitand_assign(&mut self, rhs: KernelProfilingInfo) {
        *self = *self & rhs
    }
}
impl ::std::ops::BitXor for KernelProfilingInfo {
    type Output = KernelProfilingInfo;
    #[inline]
    fn bitxor(self, rhs: KernelProfilingInfo) -> KernelProfilingInfo {
        KernelProfilingInfo(self.0 ^ rhs.0)
    }
}
impl ::std::ops::BitXorAssign for KernelProfilingInfo {
    #[inline]
    fn bitxor_assign(&mut self, rhs: KernelProfilingInfo) {
        *self = *self ^ rhs
    }
}
impl ::std::ops::Sub for KernelProfilingInfo {
    type Output = KernelProfilingInfo;
    #[inline]
    fn sub(self, rhs: KernelProfilingInfo) -> KernelProfilingInfo {
        self & !rhs
    }
}
impl ::std::ops::SubAssign for KernelProfilingInfo {
    #[inline]
    fn sub_assign(&mut self, rhs: KernelProfilingInfo) {
        *self = *self - rhs
    }
}
impl ::std::ops::Not for KernelProfilingInfo {
    type Output = KernelProfilingInfo;
    #[inline]
    fn not(self) -> KernelProfilingInfo {
        self ^ KernelProfilingInfo::all()
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u32)]
pub enum SourceLanguage {
    Unknown,
    ESSL,
    GLSL,
    OpenCL_C,
    OpenCL_CPP,
    HLSL,
}
impl SourceLanguage {
    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
        let value = data[0];
        data = &data[1..];
        match value {
            0u32 => (SourceLanguage::Unknown, data),
            1u32 => (SourceLanguage::ESSL, data),
            2u32 => (SourceLanguage::GLSL, data),
            3u32 => (SourceLanguage::OpenCL_C, data),
            4u32 => (SourceLanguage::OpenCL_CPP, data),
            5u32 => (SourceLanguage::HLSL, data),
            _ => panic!("Unknown value for Enum: {}", "SourceLanguage"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u32)]
pub enum ExecutionModel {
    Vertex,
    TessellationControl,
    TessellationEvaluation,
    Geometry,
    Fragment,
    GLCompute,
    Kernel,
    TaskNV,
    MeshNV,
    RayGenerationNV,
    IntersectionNV,
    AnyHitNV,
    ClosestHitNV,
    MissNV,
    CallableNV,
}
impl ExecutionModel {
    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
        let value = data[0];
        data = &data[1..];
        match value {
            0u32 => (ExecutionModel::Vertex, data),
            1u32 => (ExecutionModel::TessellationControl, data),
            2u32 => (ExecutionModel::TessellationEvaluation, data),
            3u32 => (ExecutionModel::Geometry, data),
            4u32 => (ExecutionModel::Fragment, data),
            5u32 => (ExecutionModel::GLCompute, data),
            6u32 => (ExecutionModel::Kernel, data),
            5267u32 => (ExecutionModel::TaskNV, data),
            5268u32 => (ExecutionModel::MeshNV, data),
            5313u32 => (ExecutionModel::RayGenerationNV, data),
            5314u32 => (ExecutionModel::IntersectionNV, data),
            5315u32 => (ExecutionModel::AnyHitNV, data),
            5316u32 => (ExecutionModel::ClosestHitNV, data),
            5317u32 => (ExecutionModel::MissNV, data),
            5318u32 => (ExecutionModel::CallableNV, data),
            _ => panic!("Unknown value for Enum: {}", "ExecutionModel"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u32)]
pub enum AddressingModel {
    Logical,
    Physical32,
    Physical64,
    PhysicalStorageBuffer64,
}
impl AddressingModel {
    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
        let value = data[0];
        data = &data[1..];
        match value {
            0u32 => (AddressingModel::Logical, data),
            1u32 => (AddressingModel::Physical32, data),
            2u32 => (AddressingModel::Physical64, data),
            5348u32 => (AddressingModel::PhysicalStorageBuffer64, data),
            _ => panic!("Unknown value for Enum: {}", "AddressingModel"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u32)]
pub enum MemoryModel {
    Simple,
    GLSL450,
    OpenCL,
    Vulkan,
}
impl MemoryModel {
    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
        let value = data[0];
        data = &data[1..];
        match value {
            0u32 => (MemoryModel::Simple, data),
            1u32 => (MemoryModel::GLSL450, data),
            2u32 => (MemoryModel::OpenCL, data),
            3u32 => (MemoryModel::Vulkan, data),
            _ => panic!("Unknown value for Enum: {}", "MemoryModel"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u32)]
pub enum ExecutionMode {
    Invocations(LiteralInteger),
    SpacingEqual,
    SpacingFractionalEven,
    SpacingFractionalOdd,
    VertexOrderCw,
    VertexOrderCcw,
    PixelCenterInteger,
    OriginUpperLeft,
    OriginLowerLeft,
    EarlyFragmentTests,
    PointMode,
    Xfb,
    DepthReplacing,
    DepthGreater,
    DepthLess,
    DepthUnchanged,
    LocalSize(LiteralInteger, LiteralInteger, LiteralInteger),
    LocalSizeHint(LiteralInteger, LiteralInteger, LiteralInteger),
    InputPoints,
    InputLines,
    InputLinesAdjacency,
    Triangles,
    InputTrianglesAdjacency,
    Quads,
    Isolines,
    OutputVertices(LiteralInteger),
    OutputPoints,
    OutputLineStrip,
    OutputTriangleStrip,
    VecTypeHint(LiteralInteger),
    ContractionOff,
    Initializer,
    Finalizer,
    SubgroupSize(LiteralInteger),
    SubgroupsPerWorkgroup(LiteralInteger),
    SubgroupsPerWorkgroupId(IdRef),
    LocalSizeId(IdRef, IdRef, IdRef),
    LocalSizeHintId(IdRef),
    PostDepthCoverage,
    DenormPreserve(LiteralInteger),
    DenormFlushToZero(LiteralInteger),
    SignedZeroInfNanPreserve(LiteralInteger),
    RoundingModeRTE(LiteralInteger),
    RoundingModeRTZ(LiteralInteger),
    StencilRefReplacingEXT,
    OutputLinesNV,
    OutputPrimitivesNV(LiteralInteger),
    DerivativeGroupQuadsNV,
    DerivativeGroupLinearNV,
    OutputTrianglesNV,
    PixelInterlockOrderedEXT,
    PixelInterlockUnorderedEXT,
    SampleInterlockOrderedEXT,
    SampleInterlockUnorderedEXT,
    ShadingRateInterlockOrderedEXT,
    ShadingRateInterlockUnorderedEXT,
}
impl ExecutionMode {
    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
        let value = data[0];
        data = &data[1..];
        match value {
            0u32 => {
                let s = ExecutionMode::Invocations({
                    let (v, d) = LiteralInteger::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            1u32 => (ExecutionMode::SpacingEqual, data),
            2u32 => (ExecutionMode::SpacingFractionalEven, data),
            3u32 => (ExecutionMode::SpacingFractionalOdd, data),
            4u32 => (ExecutionMode::VertexOrderCw, data),
            5u32 => (ExecutionMode::VertexOrderCcw, data),
            6u32 => (ExecutionMode::PixelCenterInteger, data),
            7u32 => (ExecutionMode::OriginUpperLeft, data),
            8u32 => (ExecutionMode::OriginLowerLeft, data),
            9u32 => (ExecutionMode::EarlyFragmentTests, data),
            10u32 => (ExecutionMode::PointMode, data),
            11u32 => (ExecutionMode::Xfb, data),
            12u32 => (ExecutionMode::DepthReplacing, data),
            14u32 => (ExecutionMode::DepthGreater, data),
            15u32 => (ExecutionMode::DepthLess, data),
            16u32 => (ExecutionMode::DepthUnchanged, data),
            17u32 => {
                let s = ExecutionMode::LocalSize(
                    {
                        let (v, d) = LiteralInteger::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralInteger::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralInteger::from_raw(data);
                        data = d;
                        v
                    },
                );
                (s, data)
            }
            18u32 => {
                let s = ExecutionMode::LocalSizeHint(
                    {
                        let (v, d) = LiteralInteger::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralInteger::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralInteger::from_raw(data);
                        data = d;
                        v
                    },
                );
                (s, data)
            }
            19u32 => (ExecutionMode::InputPoints, data),
            20u32 => (ExecutionMode::InputLines, data),
            21u32 => (ExecutionMode::InputLinesAdjacency, data),
            22u32 => (ExecutionMode::Triangles, data),
            23u32 => (ExecutionMode::InputTrianglesAdjacency, data),
            24u32 => (ExecutionMode::Quads, data),
            25u32 => (ExecutionMode::Isolines, data),
            26u32 => {
                let s = ExecutionMode::OutputVertices({
                    let (v, d) = LiteralInteger::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            27u32 => (ExecutionMode::OutputPoints, data),
            28u32 => (ExecutionMode::OutputLineStrip, data),
            29u32 => (ExecutionMode::OutputTriangleStrip, data),
            30u32 => {
                let s = ExecutionMode::VecTypeHint({
                    let (v, d) = LiteralInteger::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            31u32 => (ExecutionMode::ContractionOff, data),
            33u32 => (ExecutionMode::Initializer, data),
            34u32 => (ExecutionMode::Finalizer, data),
            35u32 => {
                let s = ExecutionMode::SubgroupSize({
                    let (v, d) = LiteralInteger::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            36u32 => {
                let s = ExecutionMode::SubgroupsPerWorkgroup({
                    let (v, d) = LiteralInteger::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            37u32 => {
                let s = ExecutionMode::SubgroupsPerWorkgroupId({
                    let (v, d) = IdRef::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            38u32 => {
                let s = ExecutionMode::LocalSizeId(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                (s, data)
            }
            39u32 => {
                let s = ExecutionMode::LocalSizeHintId({
                    let (v, d) = IdRef::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            4446u32 => (ExecutionMode::PostDepthCoverage, data),
            4459u32 => {
                let s = ExecutionMode::DenormPreserve({
                    let (v, d) = LiteralInteger::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            4460u32 => {
                let s = ExecutionMode::DenormFlushToZero({
                    let (v, d) = LiteralInteger::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            4461u32 => {
                let s = ExecutionMode::SignedZeroInfNanPreserve({
                    let (v, d) = LiteralInteger::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            4462u32 => {
                let s = ExecutionMode::RoundingModeRTE({
                    let (v, d) = LiteralInteger::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            4463u32 => {
                let s = ExecutionMode::RoundingModeRTZ({
                    let (v, d) = LiteralInteger::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            5027u32 => (ExecutionMode::StencilRefReplacingEXT, data),
            5269u32 => (ExecutionMode::OutputLinesNV, data),
            5270u32 => {
                let s = ExecutionMode::OutputPrimitivesNV({
                    let (v, d) = LiteralInteger::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            5289u32 => (ExecutionMode::DerivativeGroupQuadsNV, data),
            5290u32 => (ExecutionMode::DerivativeGroupLinearNV, data),
            5298u32 => (ExecutionMode::OutputTrianglesNV, data),
            5366u32 => (ExecutionMode::PixelInterlockOrderedEXT, data),
            5367u32 => (ExecutionMode::PixelInterlockUnorderedEXT, data),
            5368u32 => (ExecutionMode::SampleInterlockOrderedEXT, data),
            5369u32 => (ExecutionMode::SampleInterlockUnorderedEXT, data),
            5370u32 => (ExecutionMode::ShadingRateInterlockOrderedEXT, data),
            5371u32 => (ExecutionMode::ShadingRateInterlockUnorderedEXT, data),
            _ => panic!("Unknown value for Enum: {}", "ExecutionMode"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u32)]
pub enum StorageClass {
    UniformConstant,
    Input,
    Uniform,
    Output,
    Workgroup,
    CrossWorkgroup,
    Private,
    Function,
    Generic,
    PushConstant,
    AtomicCounter,
    Image,
    StorageBuffer,
    CallableDataNV,
    IncomingCallableDataNV,
    RayPayloadNV,
    HitAttributeNV,
    IncomingRayPayloadNV,
    ShaderRecordBufferNV,
    PhysicalStorageBuffer,
}
impl StorageClass {
    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
        let value = data[0];
        data = &data[1..];
        match value {
            0u32 => (StorageClass::UniformConstant, data),
            1u32 => (StorageClass::Input, data),
            2u32 => (StorageClass::Uniform, data),
            3u32 => (StorageClass::Output, data),
            4u32 => (StorageClass::Workgroup, data),
            5u32 => (StorageClass::CrossWorkgroup, data),
            6u32 => (StorageClass::Private, data),
            7u32 => (StorageClass::Function, data),
            8u32 => (StorageClass::Generic, data),
            9u32 => (StorageClass::PushConstant, data),
            10u32 => (StorageClass::AtomicCounter, data),
            11u32 => (StorageClass::Image, data),
            12u32 => (StorageClass::StorageBuffer, data),
            5328u32 => (StorageClass::CallableDataNV, data),
            5329u32 => (StorageClass::IncomingCallableDataNV, data),
            5338u32 => (StorageClass::RayPayloadNV, data),
            5339u32 => (StorageClass::HitAttributeNV, data),
            5342u32 => (StorageClass::IncomingRayPayloadNV, data),
            5343u32 => (StorageClass::ShaderRecordBufferNV, data),
            5349u32 => (StorageClass::PhysicalStorageBuffer, data),
            _ => panic!("Unknown value for Enum: {}", "StorageClass"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u32)]
pub enum Dim {
    _1D,
    _2D,
    _3D,
    Cube,
    Rect,
    Buffer,
    SubpassData,
}
impl Dim {
    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
        let value = data[0];
        data = &data[1..];
        match value {
            0u32 => (Dim::_1D, data),
            1u32 => (Dim::_2D, data),
            2u32 => (Dim::_3D, data),
            3u32 => (Dim::Cube, data),
            4u32 => (Dim::Rect, data),
            5u32 => (Dim::Buffer, data),
            6u32 => (Dim::SubpassData, data),
            _ => panic!("Unknown value for Enum: {}", "Dim"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u32)]
pub enum SamplerAddressingMode {
    None,
    ClampToEdge,
    Clamp,
    Repeat,
    RepeatMirrored,
}
impl SamplerAddressingMode {
    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
        let value = data[0];
        data = &data[1..];
        match value {
            0u32 => (SamplerAddressingMode::None, data),
            1u32 => (SamplerAddressingMode::ClampToEdge, data),
            2u32 => (SamplerAddressingMode::Clamp, data),
            3u32 => (SamplerAddressingMode::Repeat, data),
            4u32 => (SamplerAddressingMode::RepeatMirrored, data),
            _ => panic!("Unknown value for Enum: {}", "SamplerAddressingMode"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u32)]
pub enum SamplerFilterMode {
    Nearest,
    Linear,
}
impl SamplerFilterMode {
    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
        let value = data[0];
        data = &data[1..];
        match value {
            0u32 => (SamplerFilterMode::Nearest, data),
            1u32 => (SamplerFilterMode::Linear, data),
            _ => panic!("Unknown value for Enum: {}", "SamplerFilterMode"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u32)]
pub enum ImageFormat {
    Unknown,
    Rgba32f,
    Rgba16f,
    R32f,
    Rgba8,
    Rgba8Snorm,
    Rg32f,
    Rg16f,
    R11fG11fB10f,
    R16f,
    Rgba16,
    Rgb10A2,
    Rg16,
    Rg8,
    R16,
    R8,
    Rgba16Snorm,
    Rg16Snorm,
    Rg8Snorm,
    R16Snorm,
    R8Snorm,
    Rgba32i,
    Rgba16i,
    Rgba8i,
    R32i,
    Rg32i,
    Rg16i,
    Rg8i,
    R16i,
    R8i,
    Rgba32ui,
    Rgba16ui,
    Rgba8ui,
    R32ui,
    Rgb10a2ui,
    Rg32ui,
    Rg16ui,
    Rg8ui,
    R16ui,
    R8ui,
}
impl ImageFormat {
    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
        let value = data[0];
        data = &data[1..];
        match value {
            0u32 => (ImageFormat::Unknown, data),
            1u32 => (ImageFormat::Rgba32f, data),
            2u32 => (ImageFormat::Rgba16f, data),
            3u32 => (ImageFormat::R32f, data),
            4u32 => (ImageFormat::Rgba8, data),
            5u32 => (ImageFormat::Rgba8Snorm, data),
            6u32 => (ImageFormat::Rg32f, data),
            7u32 => (ImageFormat::Rg16f, data),
            8u32 => (ImageFormat::R11fG11fB10f, data),
            9u32 => (ImageFormat::R16f, data),
            10u32 => (ImageFormat::Rgba16, data),
            11u32 => (ImageFormat::Rgb10A2, data),
            12u32 => (ImageFormat::Rg16, data),
            13u32 => (ImageFormat::Rg8, data),
            14u32 => (ImageFormat::R16, data),
            15u32 => (ImageFormat::R8, data),
            16u32 => (ImageFormat::Rgba16Snorm, data),
            17u32 => (ImageFormat::Rg16Snorm, data),
            18u32 => (ImageFormat::Rg8Snorm, data),
            19u32 => (ImageFormat::R16Snorm, data),
            20u32 => (ImageFormat::R8Snorm, data),
            21u32 => (ImageFormat::Rgba32i, data),
            22u32 => (ImageFormat::Rgba16i, data),
            23u32 => (ImageFormat::Rgba8i, data),
            24u32 => (ImageFormat::R32i, data),
            25u32 => (ImageFormat::Rg32i, data),
            26u32 => (ImageFormat::Rg16i, data),
            27u32 => (ImageFormat::Rg8i, data),
            28u32 => (ImageFormat::R16i, data),
            29u32 => (ImageFormat::R8i, data),
            30u32 => (ImageFormat::Rgba32ui, data),
            31u32 => (ImageFormat::Rgba16ui, data),
            32u32 => (ImageFormat::Rgba8ui, data),
            33u32 => (ImageFormat::R32ui, data),
            34u32 => (ImageFormat::Rgb10a2ui, data),
            35u32 => (ImageFormat::Rg32ui, data),
            36u32 => (ImageFormat::Rg16ui, data),
            37u32 => (ImageFormat::Rg8ui, data),
            38u32 => (ImageFormat::R16ui, data),
            39u32 => (ImageFormat::R8ui, data),
            _ => panic!("Unknown value for Enum: {}", "ImageFormat"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u32)]
pub enum ImageChannelOrder {
    R,
    A,
    RG,
    RA,
    RGB,
    RGBA,
    BGRA,
    ARGB,
    Intensity,
    Luminance,
    Rx,
    RGx,
    RGBx,
    Depth,
    DepthStencil,
    sRGB,
    sRGBx,
    sRGBA,
    sBGRA,
    ABGR,
}
impl ImageChannelOrder {
    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
        let value = data[0];
        data = &data[1..];
        match value {
            0u32 => (ImageChannelOrder::R, data),
            1u32 => (ImageChannelOrder::A, data),
            2u32 => (ImageChannelOrder::RG, data),
            3u32 => (ImageChannelOrder::RA, data),
            4u32 => (ImageChannelOrder::RGB, data),
            5u32 => (ImageChannelOrder::RGBA, data),
            6u32 => (ImageChannelOrder::BGRA, data),
            7u32 => (ImageChannelOrder::ARGB, data),
            8u32 => (ImageChannelOrder::Intensity, data),
            9u32 => (ImageChannelOrder::Luminance, data),
            10u32 => (ImageChannelOrder::Rx, data),
            11u32 => (ImageChannelOrder::RGx, data),
            12u32 => (ImageChannelOrder::RGBx, data),
            13u32 => (ImageChannelOrder::Depth, data),
            14u32 => (ImageChannelOrder::DepthStencil, data),
            15u32 => (ImageChannelOrder::sRGB, data),
            16u32 => (ImageChannelOrder::sRGBx, data),
            17u32 => (ImageChannelOrder::sRGBA, data),
            18u32 => (ImageChannelOrder::sBGRA, data),
            19u32 => (ImageChannelOrder::ABGR, data),
            _ => panic!("Unknown value for Enum: {}", "ImageChannelOrder"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u32)]
pub enum ImageChannelDataType {
    SnormInt8,
    SnormInt16,
    UnormInt8,
    UnormInt16,
    UnormShort565,
    UnormShort555,
    UnormInt101010,
    SignedInt8,
    SignedInt16,
    SignedInt32,
    UnsignedInt8,
    UnsignedInt16,
    UnsignedInt32,
    HalfFloat,
    Float,
    UnormInt24,
    UnormInt101010_2,
}
impl ImageChannelDataType {
    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
        let value = data[0];
        data = &data[1..];
        match value {
            0u32 => (ImageChannelDataType::SnormInt8, data),
            1u32 => (ImageChannelDataType::SnormInt16, data),
            2u32 => (ImageChannelDataType::UnormInt8, data),
            3u32 => (ImageChannelDataType::UnormInt16, data),
            4u32 => (ImageChannelDataType::UnormShort565, data),
            5u32 => (ImageChannelDataType::UnormShort555, data),
            6u32 => (ImageChannelDataType::UnormInt101010, data),
            7u32 => (ImageChannelDataType::SignedInt8, data),
            8u32 => (ImageChannelDataType::SignedInt16, data),
            9u32 => (ImageChannelDataType::SignedInt32, data),
            10u32 => (ImageChannelDataType::UnsignedInt8, data),
            11u32 => (ImageChannelDataType::UnsignedInt16, data),
            12u32 => (ImageChannelDataType::UnsignedInt32, data),
            13u32 => (ImageChannelDataType::HalfFloat, data),
            14u32 => (ImageChannelDataType::Float, data),
            15u32 => (ImageChannelDataType::UnormInt24, data),
            16u32 => (ImageChannelDataType::UnormInt101010_2, data),
            _ => panic!("Unknown value for Enum: {}", "ImageChannelDataType"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u32)]
pub enum FPRoundingMode {
    RTE,
    RTZ,
    RTP,
    RTN,
}
impl FPRoundingMode {
    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
        let value = data[0];
        data = &data[1..];
        match value {
            0u32 => (FPRoundingMode::RTE, data),
            1u32 => (FPRoundingMode::RTZ, data),
            2u32 => (FPRoundingMode::RTP, data),
            3u32 => (FPRoundingMode::RTN, data),
            _ => panic!("Unknown value for Enum: {}", "FPRoundingMode"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u32)]
pub enum LinkageType {
    Export,
    Import,
}
impl LinkageType {
    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
        let value = data[0];
        data = &data[1..];
        match value {
            0u32 => (LinkageType::Export, data),
            1u32 => (LinkageType::Import, data),
            _ => panic!("Unknown value for Enum: {}", "LinkageType"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u32)]
pub enum AccessQualifier {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}
impl AccessQualifier {
    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
        let value = data[0];
        data = &data[1..];
        match value {
            0u32 => (AccessQualifier::ReadOnly, data),
            1u32 => (AccessQualifier::WriteOnly, data),
            2u32 => (AccessQualifier::ReadWrite, data),
            _ => panic!("Unknown value for Enum: {}", "AccessQualifier"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u32)]
pub enum FunctionParameterAttribute {
    Zext,
    Sext,
    ByVal,
    Sret,
    NoAlias,
    NoCapture,
    NoWrite,
    NoReadWrite,
}
impl FunctionParameterAttribute {
    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
        let value = data[0];
        data = &data[1..];
        match value {
            0u32 => (FunctionParameterAttribute::Zext, data),
            1u32 => (FunctionParameterAttribute::Sext, data),
            2u32 => (FunctionParameterAttribute::ByVal, data),
            3u32 => (FunctionParameterAttribute::Sret, data),
            4u32 => (FunctionParameterAttribute::NoAlias, data),
            5u32 => (FunctionParameterAttribute::NoCapture, data),
            6u32 => (FunctionParameterAttribute::NoWrite, data),
            7u32 => (FunctionParameterAttribute::NoReadWrite, data),
            _ => panic!("Unknown value for Enum: {}", "FunctionParameterAttribute"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u32)]
pub enum Decoration {
    RelaxedPrecision,
    SpecId(LiteralInteger),
    Block,
    BufferBlock,
    RowMajor,
    ColMajor,
    ArrayStride(LiteralInteger),
    MatrixStride(LiteralInteger),
    GLSLShared,
    GLSLPacked,
    CPacked,
    BuiltIn(BuiltIn),
    NoPerspective,
    Flat,
    Patch,
    Centroid,
    Sample,
    Invariant,
    Restrict,
    Aliased,
    Volatile,
    Constant,
    Coherent,
    NonWritable,
    NonReadable,
    Uniform,
    UniformId(IdScope),
    SaturatedConversion,
    Stream(LiteralInteger),
    Location(LiteralInteger),
    Component(LiteralInteger),
    Index(LiteralInteger),
    Binding(LiteralInteger),
    DescriptorSet(LiteralInteger),
    Offset(LiteralInteger),
    XfbBuffer(LiteralInteger),
    XfbStride(LiteralInteger),
    FuncParamAttr(FunctionParameterAttribute),
    FPRoundingMode(FPRoundingMode),
    FPFastMathMode(FPFastMathMode),
    LinkageAttributes(LiteralString, LinkageType),
    NoContraction,
    InputAttachmentIndex(LiteralInteger),
    Alignment(LiteralInteger),
    MaxByteOffset(LiteralInteger),
    AlignmentId(IdRef),
    MaxByteOffsetId(IdRef),
    NoSignedWrap,
    NoUnsignedWrap,
    ExplicitInterpAMD,
    OverrideCoverageNV,
    PassthroughNV,
    ViewportRelativeNV,
    SecondaryViewportRelativeNV(LiteralInteger),
    PerPrimitiveNV,
    PerViewNV,
    PerTaskNV,
    PerVertexNV,
    NonUniform,
    RestrictPointer,
    AliasedPointer,
    CounterBuffer(IdRef),
    UserSemantic(LiteralString),
    UserTypeGOOGLE(LiteralString),
}
impl Decoration {
    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
        let value = data[0];
        data = &data[1..];
        match value {
            0u32 => (Decoration::RelaxedPrecision, data),
            1u32 => {
                let s = Decoration::SpecId({
                    let (v, d) = LiteralInteger::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            2u32 => (Decoration::Block, data),
            3u32 => (Decoration::BufferBlock, data),
            4u32 => (Decoration::RowMajor, data),
            5u32 => (Decoration::ColMajor, data),
            6u32 => {
                let s = Decoration::ArrayStride({
                    let (v, d) = LiteralInteger::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            7u32 => {
                let s = Decoration::MatrixStride({
                    let (v, d) = LiteralInteger::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            8u32 => (Decoration::GLSLShared, data),
            9u32 => (Decoration::GLSLPacked, data),
            10u32 => (Decoration::CPacked, data),
            11u32 => {
                let s = Decoration::BuiltIn({
                    let (v, d) = BuiltIn::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            13u32 => (Decoration::NoPerspective, data),
            14u32 => (Decoration::Flat, data),
            15u32 => (Decoration::Patch, data),
            16u32 => (Decoration::Centroid, data),
            17u32 => (Decoration::Sample, data),
            18u32 => (Decoration::Invariant, data),
            19u32 => (Decoration::Restrict, data),
            20u32 => (Decoration::Aliased, data),
            21u32 => (Decoration::Volatile, data),
            22u32 => (Decoration::Constant, data),
            23u32 => (Decoration::Coherent, data),
            24u32 => (Decoration::NonWritable, data),
            25u32 => (Decoration::NonReadable, data),
            26u32 => (Decoration::Uniform, data),
            27u32 => {
                let s = Decoration::UniformId({
                    let (v, d) = IdScope::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            28u32 => (Decoration::SaturatedConversion, data),
            29u32 => {
                let s = Decoration::Stream({
                    let (v, d) = LiteralInteger::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            30u32 => {
                let s = Decoration::Location({
                    let (v, d) = LiteralInteger::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            31u32 => {
                let s = Decoration::Component({
                    let (v, d) = LiteralInteger::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            32u32 => {
                let s = Decoration::Index({
                    let (v, d) = LiteralInteger::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            33u32 => {
                let s = Decoration::Binding({
                    let (v, d) = LiteralInteger::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            34u32 => {
                let s = Decoration::DescriptorSet({
                    let (v, d) = LiteralInteger::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            35u32 => {
                let s = Decoration::Offset({
                    let (v, d) = LiteralInteger::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            36u32 => {
                let s = Decoration::XfbBuffer({
                    let (v, d) = LiteralInteger::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            37u32 => {
                let s = Decoration::XfbStride({
                    let (v, d) = LiteralInteger::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            38u32 => {
                let s = Decoration::FuncParamAttr({
                    let (v, d) = FunctionParameterAttribute::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            39u32 => {
                let s = Decoration::FPRoundingMode({
                    let (v, d) = FPRoundingMode::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            40u32 => {
                let s = Decoration::FPFastMathMode({
                    let (v, d) = FPFastMathMode::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            41u32 => {
                let s = Decoration::LinkageAttributes(
                    {
                        let (v, d) = LiteralString::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LinkageType::from_raw(data);
                        data = d;
                        v
                    },
                );
                (s, data)
            }
            42u32 => (Decoration::NoContraction, data),
            43u32 => {
                let s = Decoration::InputAttachmentIndex({
                    let (v, d) = LiteralInteger::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            44u32 => {
                let s = Decoration::Alignment({
                    let (v, d) = LiteralInteger::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            45u32 => {
                let s = Decoration::MaxByteOffset({
                    let (v, d) = LiteralInteger::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            46u32 => {
                let s = Decoration::AlignmentId({
                    let (v, d) = IdRef::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            47u32 => {
                let s = Decoration::MaxByteOffsetId({
                    let (v, d) = IdRef::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            4469u32 => (Decoration::NoSignedWrap, data),
            4470u32 => (Decoration::NoUnsignedWrap, data),
            4999u32 => (Decoration::ExplicitInterpAMD, data),
            5248u32 => (Decoration::OverrideCoverageNV, data),
            5250u32 => (Decoration::PassthroughNV, data),
            5252u32 => (Decoration::ViewportRelativeNV, data),
            5256u32 => {
                let s = Decoration::SecondaryViewportRelativeNV({
                    let (v, d) = LiteralInteger::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            5271u32 => (Decoration::PerPrimitiveNV, data),
            5272u32 => (Decoration::PerViewNV, data),
            5273u32 => (Decoration::PerTaskNV, data),
            5285u32 => (Decoration::PerVertexNV, data),
            5300u32 => (Decoration::NonUniform, data),
            5355u32 => (Decoration::RestrictPointer, data),
            5356u32 => (Decoration::AliasedPointer, data),
            5634u32 => {
                let s = Decoration::CounterBuffer({
                    let (v, d) = IdRef::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            5635u32 => {
                let s = Decoration::UserSemantic({
                    let (v, d) = LiteralString::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            5636u32 => {
                let s = Decoration::UserTypeGOOGLE({
                    let (v, d) = LiteralString::from_raw(data);
                    data = d;
                    v
                });
                (s, data)
            }
            _ => panic!("Unknown value for Enum: {}", "Decoration"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u32)]
pub enum BuiltIn {
    Position,
    PointSize,
    ClipDistance,
    CullDistance,
    VertexId,
    InstanceId,
    PrimitiveId,
    InvocationId,
    Layer,
    ViewportIndex,
    TessLevelOuter,
    TessLevelInner,
    TessCoord,
    PatchVertices,
    FragCoord,
    PointCoord,
    FrontFacing,
    SampleId,
    SamplePosition,
    SampleMask,
    FragDepth,
    HelperInvocation,
    NumWorkgroups,
    WorkgroupSize,
    WorkgroupId,
    LocalInvocationId,
    GlobalInvocationId,
    LocalInvocationIndex,
    WorkDim,
    GlobalSize,
    EnqueuedWorkgroupSize,
    GlobalOffset,
    GlobalLinearId,
    SubgroupSize,
    SubgroupMaxSize,
    NumSubgroups,
    NumEnqueuedSubgroups,
    SubgroupId,
    SubgroupLocalInvocationId,
    VertexIndex,
    InstanceIndex,
    SubgroupEqMask,
    SubgroupGeMask,
    SubgroupGtMask,
    SubgroupLeMask,
    SubgroupLtMask,
    BaseVertex,
    BaseInstance,
    DrawIndex,
    DeviceIndex,
    ViewIndex,
    BaryCoordNoPerspAMD,
    BaryCoordNoPerspCentroidAMD,
    BaryCoordNoPerspSampleAMD,
    BaryCoordSmoothAMD,
    BaryCoordSmoothCentroidAMD,
    BaryCoordSmoothSampleAMD,
    BaryCoordPullModelAMD,
    FragStencilRefEXT,
    ViewportMaskNV,
    SecondaryPositionNV,
    SecondaryViewportMaskNV,
    PositionPerViewNV,
    ViewportMaskPerViewNV,
    FullyCoveredEXT,
    TaskCountNV,
    PrimitiveCountNV,
    PrimitiveIndicesNV,
    ClipDistancePerViewNV,
    CullDistancePerViewNV,
    LayerPerViewNV,
    MeshViewCountNV,
    MeshViewIndicesNV,
    BaryCoordNV,
    BaryCoordNoPerspNV,
    FragSizeEXT,
    FragInvocationCountEXT,
    LaunchIdNV,
    LaunchSizeNV,
    WorldRayOriginNV,
    WorldRayDirectionNV,
    ObjectRayOriginNV,
    ObjectRayDirectionNV,
    RayTminNV,
    RayTmaxNV,
    InstanceCustomIndexNV,
    ObjectToWorldNV,
    WorldToObjectNV,
    HitTNV,
    HitKindNV,
    IncomingRayFlagsNV,
    WarpsPerSMNV,
    SMCountNV,
    WarpIDNV,
    SMIDNV,
}
impl BuiltIn {
    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
        let value = data[0];
        data = &data[1..];
        match value {
            0u32 => (BuiltIn::Position, data),
            1u32 => (BuiltIn::PointSize, data),
            3u32 => (BuiltIn::ClipDistance, data),
            4u32 => (BuiltIn::CullDistance, data),
            5u32 => (BuiltIn::VertexId, data),
            6u32 => (BuiltIn::InstanceId, data),
            7u32 => (BuiltIn::PrimitiveId, data),
            8u32 => (BuiltIn::InvocationId, data),
            9u32 => (BuiltIn::Layer, data),
            10u32 => (BuiltIn::ViewportIndex, data),
            11u32 => (BuiltIn::TessLevelOuter, data),
            12u32 => (BuiltIn::TessLevelInner, data),
            13u32 => (BuiltIn::TessCoord, data),
            14u32 => (BuiltIn::PatchVertices, data),
            15u32 => (BuiltIn::FragCoord, data),
            16u32 => (BuiltIn::PointCoord, data),
            17u32 => (BuiltIn::FrontFacing, data),
            18u32 => (BuiltIn::SampleId, data),
            19u32 => (BuiltIn::SamplePosition, data),
            20u32 => (BuiltIn::SampleMask, data),
            22u32 => (BuiltIn::FragDepth, data),
            23u32 => (BuiltIn::HelperInvocation, data),
            24u32 => (BuiltIn::NumWorkgroups, data),
            25u32 => (BuiltIn::WorkgroupSize, data),
            26u32 => (BuiltIn::WorkgroupId, data),
            27u32 => (BuiltIn::LocalInvocationId, data),
            28u32 => (BuiltIn::GlobalInvocationId, data),
            29u32 => (BuiltIn::LocalInvocationIndex, data),
            30u32 => (BuiltIn::WorkDim, data),
            31u32 => (BuiltIn::GlobalSize, data),
            32u32 => (BuiltIn::EnqueuedWorkgroupSize, data),
            33u32 => (BuiltIn::GlobalOffset, data),
            34u32 => (BuiltIn::GlobalLinearId, data),
            36u32 => (BuiltIn::SubgroupSize, data),
            37u32 => (BuiltIn::SubgroupMaxSize, data),
            38u32 => (BuiltIn::NumSubgroups, data),
            39u32 => (BuiltIn::NumEnqueuedSubgroups, data),
            40u32 => (BuiltIn::SubgroupId, data),
            41u32 => (BuiltIn::SubgroupLocalInvocationId, data),
            42u32 => (BuiltIn::VertexIndex, data),
            43u32 => (BuiltIn::InstanceIndex, data),
            4416u32 => (BuiltIn::SubgroupEqMask, data),
            4417u32 => (BuiltIn::SubgroupGeMask, data),
            4418u32 => (BuiltIn::SubgroupGtMask, data),
            4419u32 => (BuiltIn::SubgroupLeMask, data),
            4420u32 => (BuiltIn::SubgroupLtMask, data),
            4424u32 => (BuiltIn::BaseVertex, data),
            4425u32 => (BuiltIn::BaseInstance, data),
            4426u32 => (BuiltIn::DrawIndex, data),
            4438u32 => (BuiltIn::DeviceIndex, data),
            4440u32 => (BuiltIn::ViewIndex, data),
            4992u32 => (BuiltIn::BaryCoordNoPerspAMD, data),
            4993u32 => (BuiltIn::BaryCoordNoPerspCentroidAMD, data),
            4994u32 => (BuiltIn::BaryCoordNoPerspSampleAMD, data),
            4995u32 => (BuiltIn::BaryCoordSmoothAMD, data),
            4996u32 => (BuiltIn::BaryCoordSmoothCentroidAMD, data),
            4997u32 => (BuiltIn::BaryCoordSmoothSampleAMD, data),
            4998u32 => (BuiltIn::BaryCoordPullModelAMD, data),
            5014u32 => (BuiltIn::FragStencilRefEXT, data),
            5253u32 => (BuiltIn::ViewportMaskNV, data),
            5257u32 => (BuiltIn::SecondaryPositionNV, data),
            5258u32 => (BuiltIn::SecondaryViewportMaskNV, data),
            5261u32 => (BuiltIn::PositionPerViewNV, data),
            5262u32 => (BuiltIn::ViewportMaskPerViewNV, data),
            5264u32 => (BuiltIn::FullyCoveredEXT, data),
            5274u32 => (BuiltIn::TaskCountNV, data),
            5275u32 => (BuiltIn::PrimitiveCountNV, data),
            5276u32 => (BuiltIn::PrimitiveIndicesNV, data),
            5277u32 => (BuiltIn::ClipDistancePerViewNV, data),
            5278u32 => (BuiltIn::CullDistancePerViewNV, data),
            5279u32 => (BuiltIn::LayerPerViewNV, data),
            5280u32 => (BuiltIn::MeshViewCountNV, data),
            5281u32 => (BuiltIn::MeshViewIndicesNV, data),
            5286u32 => (BuiltIn::BaryCoordNV, data),
            5287u32 => (BuiltIn::BaryCoordNoPerspNV, data),
            5292u32 => (BuiltIn::FragSizeEXT, data),
            5293u32 => (BuiltIn::FragInvocationCountEXT, data),
            5319u32 => (BuiltIn::LaunchIdNV, data),
            5320u32 => (BuiltIn::LaunchSizeNV, data),
            5321u32 => (BuiltIn::WorldRayOriginNV, data),
            5322u32 => (BuiltIn::WorldRayDirectionNV, data),
            5323u32 => (BuiltIn::ObjectRayOriginNV, data),
            5324u32 => (BuiltIn::ObjectRayDirectionNV, data),
            5325u32 => (BuiltIn::RayTminNV, data),
            5326u32 => (BuiltIn::RayTmaxNV, data),
            5327u32 => (BuiltIn::InstanceCustomIndexNV, data),
            5330u32 => (BuiltIn::ObjectToWorldNV, data),
            5331u32 => (BuiltIn::WorldToObjectNV, data),
            5332u32 => (BuiltIn::HitTNV, data),
            5333u32 => (BuiltIn::HitKindNV, data),
            5351u32 => (BuiltIn::IncomingRayFlagsNV, data),
            5374u32 => (BuiltIn::WarpsPerSMNV, data),
            5375u32 => (BuiltIn::SMCountNV, data),
            5376u32 => (BuiltIn::WarpIDNV, data),
            5377u32 => (BuiltIn::SMIDNV, data),
            _ => panic!("Unknown value for Enum: {}", "BuiltIn"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u32)]
pub enum Scope {
    CrossDevice,
    Device,
    Workgroup,
    Subgroup,
    Invocation,
    QueueFamily,
}
impl Scope {
    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
        let value = data[0];
        data = &data[1..];
        match value {
            0u32 => (Scope::CrossDevice, data),
            1u32 => (Scope::Device, data),
            2u32 => (Scope::Workgroup, data),
            3u32 => (Scope::Subgroup, data),
            4u32 => (Scope::Invocation, data),
            5u32 => (Scope::QueueFamily, data),
            _ => panic!("Unknown value for Enum: {}", "Scope"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u32)]
pub enum GroupOperation {
    Reduce,
    InclusiveScan,
    ExclusiveScan,
    ClusteredReduce,
    PartitionedReduceNV,
    PartitionedInclusiveScanNV,
    PartitionedExclusiveScanNV,
}
impl GroupOperation {
    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
        let value = data[0];
        data = &data[1..];
        match value {
            0u32 => (GroupOperation::Reduce, data),
            1u32 => (GroupOperation::InclusiveScan, data),
            2u32 => (GroupOperation::ExclusiveScan, data),
            3u32 => (GroupOperation::ClusteredReduce, data),
            6u32 => (GroupOperation::PartitionedReduceNV, data),
            7u32 => (GroupOperation::PartitionedInclusiveScanNV, data),
            8u32 => (GroupOperation::PartitionedExclusiveScanNV, data),
            _ => panic!("Unknown value for Enum: {}", "GroupOperation"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u32)]
pub enum KernelEnqueueFlags {
    NoWait,
    WaitKernel,
    WaitWorkGroup,
}
impl KernelEnqueueFlags {
    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
        let value = data[0];
        data = &data[1..];
        match value {
            0u32 => (KernelEnqueueFlags::NoWait, data),
            1u32 => (KernelEnqueueFlags::WaitKernel, data),
            2u32 => (KernelEnqueueFlags::WaitWorkGroup, data),
            _ => panic!("Unknown value for Enum: {}", "KernelEnqueueFlags"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u32)]
pub enum Capability {
    Matrix,
    Shader,
    Geometry,
    Tessellation,
    Addresses,
    Linkage,
    Kernel,
    Vector16,
    Float16Buffer,
    Float16,
    Float64,
    Int64,
    Int64Atomics,
    ImageBasic,
    ImageReadWrite,
    ImageMipmap,
    Pipes,
    Groups,
    DeviceEnqueue,
    LiteralSampler,
    AtomicStorage,
    Int16,
    TessellationPointSize,
    GeometryPointSize,
    ImageGatherExtended,
    StorageImageMultisample,
    UniformBufferArrayDynamicIndexing,
    SampledImageArrayDynamicIndexing,
    StorageBufferArrayDynamicIndexing,
    StorageImageArrayDynamicIndexing,
    ClipDistance,
    CullDistance,
    ImageCubeArray,
    SampleRateShading,
    ImageRect,
    SampledRect,
    GenericPointer,
    Int8,
    InputAttachment,
    SparseResidency,
    MinLod,
    Sampled1D,
    Image1D,
    SampledCubeArray,
    SampledBuffer,
    ImageBuffer,
    ImageMSArray,
    StorageImageExtendedFormats,
    ImageQuery,
    DerivativeControl,
    InterpolationFunction,
    TransformFeedback,
    GeometryStreams,
    StorageImageReadWithoutFormat,
    StorageImageWriteWithoutFormat,
    MultiViewport,
    SubgroupDispatch,
    NamedBarrier,
    PipeStorage,
    GroupNonUniform,
    GroupNonUniformVote,
    GroupNonUniformArithmetic,
    GroupNonUniformBallot,
    GroupNonUniformShuffle,
    GroupNonUniformShuffleRelative,
    GroupNonUniformClustered,
    GroupNonUniformQuad,
    ShaderLayer,
    ShaderViewportIndex,
    SubgroupBallotKHR,
    DrawParameters,
    SubgroupVoteKHR,
    StorageBuffer16BitAccess,
    UniformAndStorageBuffer16BitAccess,
    StoragePushConstant16,
    StorageInputOutput16,
    DeviceGroup,
    MultiView,
    VariablePointersStorageBuffer,
    VariablePointers,
    AtomicStorageOps,
    SampleMaskPostDepthCoverage,
    StorageBuffer8BitAccess,
    UniformAndStorageBuffer8BitAccess,
    StoragePushConstant8,
    DenormPreserve,
    DenormFlushToZero,
    SignedZeroInfNanPreserve,
    RoundingModeRTE,
    RoundingModeRTZ,
    Float16ImageAMD,
    ImageGatherBiasLodAMD,
    FragmentMaskAMD,
    StencilExportEXT,
    ImageReadWriteLodAMD,
    ShaderClockKHR,
    SampleMaskOverrideCoverageNV,
    GeometryShaderPassthroughNV,
    ShaderViewportIndexLayerEXT,
    ShaderViewportMaskNV,
    ShaderStereoViewNV,
    PerViewAttributesNV,
    FragmentFullyCoveredEXT,
    MeshShadingNV,
    ImageFootprintNV,
    FragmentBarycentricNV,
    ComputeDerivativeGroupQuadsNV,
    FragmentDensityEXT,
    GroupNonUniformPartitionedNV,
    ShaderNonUniform,
    RuntimeDescriptorArray,
    InputAttachmentArrayDynamicIndexing,
    UniformTexelBufferArrayDynamicIndexing,
    StorageTexelBufferArrayDynamicIndexing,
    UniformBufferArrayNonUniformIndexing,
    SampledImageArrayNonUniformIndexing,
    StorageBufferArrayNonUniformIndexing,
    StorageImageArrayNonUniformIndexing,
    InputAttachmentArrayNonUniformIndexing,
    UniformTexelBufferArrayNonUniformIndexing,
    StorageTexelBufferArrayNonUniformIndexing,
    RayTracingNV,
    VulkanMemoryModel,
    VulkanMemoryModelDeviceScope,
    PhysicalStorageBufferAddresses,
    ComputeDerivativeGroupLinearNV,
    CooperativeMatrixNV,
    FragmentShaderSampleInterlockEXT,
    FragmentShaderShadingRateInterlockEXT,
    ShaderSMBuiltinsNV,
    FragmentShaderPixelInterlockEXT,
    DemoteToHelperInvocationEXT,
    SubgroupShuffleINTEL,
    SubgroupBufferBlockIOINTEL,
    SubgroupImageBlockIOINTEL,
    SubgroupImageMediaBlockIOINTEL,
    IntegerFunctions2INTEL,
    SubgroupAvcMotionEstimationINTEL,
    SubgroupAvcMotionEstimationIntraINTEL,
    SubgroupAvcMotionEstimationChromaINTEL,
}
impl Capability {
    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
        let value = data[0];
        data = &data[1..];
        match value {
            0u32 => (Capability::Matrix, data),
            1u32 => (Capability::Shader, data),
            2u32 => (Capability::Geometry, data),
            3u32 => (Capability::Tessellation, data),
            4u32 => (Capability::Addresses, data),
            5u32 => (Capability::Linkage, data),
            6u32 => (Capability::Kernel, data),
            7u32 => (Capability::Vector16, data),
            8u32 => (Capability::Float16Buffer, data),
            9u32 => (Capability::Float16, data),
            10u32 => (Capability::Float64, data),
            11u32 => (Capability::Int64, data),
            12u32 => (Capability::Int64Atomics, data),
            13u32 => (Capability::ImageBasic, data),
            14u32 => (Capability::ImageReadWrite, data),
            15u32 => (Capability::ImageMipmap, data),
            17u32 => (Capability::Pipes, data),
            18u32 => (Capability::Groups, data),
            19u32 => (Capability::DeviceEnqueue, data),
            20u32 => (Capability::LiteralSampler, data),
            21u32 => (Capability::AtomicStorage, data),
            22u32 => (Capability::Int16, data),
            23u32 => (Capability::TessellationPointSize, data),
            24u32 => (Capability::GeometryPointSize, data),
            25u32 => (Capability::ImageGatherExtended, data),
            27u32 => (Capability::StorageImageMultisample, data),
            28u32 => (Capability::UniformBufferArrayDynamicIndexing, data),
            29u32 => (Capability::SampledImageArrayDynamicIndexing, data),
            30u32 => (Capability::StorageBufferArrayDynamicIndexing, data),
            31u32 => (Capability::StorageImageArrayDynamicIndexing, data),
            32u32 => (Capability::ClipDistance, data),
            33u32 => (Capability::CullDistance, data),
            34u32 => (Capability::ImageCubeArray, data),
            35u32 => (Capability::SampleRateShading, data),
            36u32 => (Capability::ImageRect, data),
            37u32 => (Capability::SampledRect, data),
            38u32 => (Capability::GenericPointer, data),
            39u32 => (Capability::Int8, data),
            40u32 => (Capability::InputAttachment, data),
            41u32 => (Capability::SparseResidency, data),
            42u32 => (Capability::MinLod, data),
            43u32 => (Capability::Sampled1D, data),
            44u32 => (Capability::Image1D, data),
            45u32 => (Capability::SampledCubeArray, data),
            46u32 => (Capability::SampledBuffer, data),
            47u32 => (Capability::ImageBuffer, data),
            48u32 => (Capability::ImageMSArray, data),
            49u32 => (Capability::StorageImageExtendedFormats, data),
            50u32 => (Capability::ImageQuery, data),
            51u32 => (Capability::DerivativeControl, data),
            52u32 => (Capability::InterpolationFunction, data),
            53u32 => (Capability::TransformFeedback, data),
            54u32 => (Capability::GeometryStreams, data),
            55u32 => (Capability::StorageImageReadWithoutFormat, data),
            56u32 => (Capability::StorageImageWriteWithoutFormat, data),
            57u32 => (Capability::MultiViewport, data),
            58u32 => (Capability::SubgroupDispatch, data),
            59u32 => (Capability::NamedBarrier, data),
            60u32 => (Capability::PipeStorage, data),
            61u32 => (Capability::GroupNonUniform, data),
            62u32 => (Capability::GroupNonUniformVote, data),
            63u32 => (Capability::GroupNonUniformArithmetic, data),
            64u32 => (Capability::GroupNonUniformBallot, data),
            65u32 => (Capability::GroupNonUniformShuffle, data),
            66u32 => (Capability::GroupNonUniformShuffleRelative, data),
            67u32 => (Capability::GroupNonUniformClustered, data),
            68u32 => (Capability::GroupNonUniformQuad, data),
            69u32 => (Capability::ShaderLayer, data),
            70u32 => (Capability::ShaderViewportIndex, data),
            4423u32 => (Capability::SubgroupBallotKHR, data),
            4427u32 => (Capability::DrawParameters, data),
            4431u32 => (Capability::SubgroupVoteKHR, data),
            4433u32 => (Capability::StorageBuffer16BitAccess, data),
            4434u32 => (Capability::UniformAndStorageBuffer16BitAccess, data),
            4435u32 => (Capability::StoragePushConstant16, data),
            4436u32 => (Capability::StorageInputOutput16, data),
            4437u32 => (Capability::DeviceGroup, data),
            4439u32 => (Capability::MultiView, data),
            4441u32 => (Capability::VariablePointersStorageBuffer, data),
            4442u32 => (Capability::VariablePointers, data),
            4445u32 => (Capability::AtomicStorageOps, data),
            4447u32 => (Capability::SampleMaskPostDepthCoverage, data),
            4448u32 => (Capability::StorageBuffer8BitAccess, data),
            4449u32 => (Capability::UniformAndStorageBuffer8BitAccess, data),
            4450u32 => (Capability::StoragePushConstant8, data),
            4464u32 => (Capability::DenormPreserve, data),
            4465u32 => (Capability::DenormFlushToZero, data),
            4466u32 => (Capability::SignedZeroInfNanPreserve, data),
            4467u32 => (Capability::RoundingModeRTE, data),
            4468u32 => (Capability::RoundingModeRTZ, data),
            5008u32 => (Capability::Float16ImageAMD, data),
            5009u32 => (Capability::ImageGatherBiasLodAMD, data),
            5010u32 => (Capability::FragmentMaskAMD, data),
            5013u32 => (Capability::StencilExportEXT, data),
            5015u32 => (Capability::ImageReadWriteLodAMD, data),
            5055u32 => (Capability::ShaderClockKHR, data),
            5249u32 => (Capability::SampleMaskOverrideCoverageNV, data),
            5251u32 => (Capability::GeometryShaderPassthroughNV, data),
            5254u32 => (Capability::ShaderViewportIndexLayerEXT, data),
            5255u32 => (Capability::ShaderViewportMaskNV, data),
            5259u32 => (Capability::ShaderStereoViewNV, data),
            5260u32 => (Capability::PerViewAttributesNV, data),
            5265u32 => (Capability::FragmentFullyCoveredEXT, data),
            5266u32 => (Capability::MeshShadingNV, data),
            5282u32 => (Capability::ImageFootprintNV, data),
            5284u32 => (Capability::FragmentBarycentricNV, data),
            5288u32 => (Capability::ComputeDerivativeGroupQuadsNV, data),
            5291u32 => (Capability::FragmentDensityEXT, data),
            5297u32 => (Capability::GroupNonUniformPartitionedNV, data),
            5301u32 => (Capability::ShaderNonUniform, data),
            5302u32 => (Capability::RuntimeDescriptorArray, data),
            5303u32 => (Capability::InputAttachmentArrayDynamicIndexing, data),
            5304u32 => (Capability::UniformTexelBufferArrayDynamicIndexing, data),
            5305u32 => (Capability::StorageTexelBufferArrayDynamicIndexing, data),
            5306u32 => (Capability::UniformBufferArrayNonUniformIndexing, data),
            5307u32 => (Capability::SampledImageArrayNonUniformIndexing, data),
            5308u32 => (Capability::StorageBufferArrayNonUniformIndexing, data),
            5309u32 => (Capability::StorageImageArrayNonUniformIndexing, data),
            5310u32 => (Capability::InputAttachmentArrayNonUniformIndexing, data),
            5311u32 => (Capability::UniformTexelBufferArrayNonUniformIndexing, data),
            5312u32 => (Capability::StorageTexelBufferArrayNonUniformIndexing, data),
            5340u32 => (Capability::RayTracingNV, data),
            5345u32 => (Capability::VulkanMemoryModel, data),
            5346u32 => (Capability::VulkanMemoryModelDeviceScope, data),
            5347u32 => (Capability::PhysicalStorageBufferAddresses, data),
            5350u32 => (Capability::ComputeDerivativeGroupLinearNV, data),
            5357u32 => (Capability::CooperativeMatrixNV, data),
            5363u32 => (Capability::FragmentShaderSampleInterlockEXT, data),
            5372u32 => (Capability::FragmentShaderShadingRateInterlockEXT, data),
            5373u32 => (Capability::ShaderSMBuiltinsNV, data),
            5378u32 => (Capability::FragmentShaderPixelInterlockEXT, data),
            5379u32 => (Capability::DemoteToHelperInvocationEXT, data),
            5568u32 => (Capability::SubgroupShuffleINTEL, data),
            5569u32 => (Capability::SubgroupBufferBlockIOINTEL, data),
            5570u32 => (Capability::SubgroupImageBlockIOINTEL, data),
            5579u32 => (Capability::SubgroupImageMediaBlockIOINTEL, data),
            5584u32 => (Capability::IntegerFunctions2INTEL, data),
            5696u32 => (Capability::SubgroupAvcMotionEstimationINTEL, data),
            5697u32 => (Capability::SubgroupAvcMotionEstimationIntraINTEL, data),
            5698u32 => (Capability::SubgroupAvcMotionEstimationChromaINTEL, data),
            _ => panic!("Unknown value for Enum: {}", "Capability"),
        }
    }
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IdResultType(pub u32);
impl IdResultType {
    pub fn from_raw(data: &[u32]) -> (Self, &[u32]) {
        (Self(data[0]), &data[1..])
    }
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IdResult(pub u32);
impl IdResult {
    pub fn from_raw(data: &[u32]) -> (Self, &[u32]) {
        (Self(data[0]), &data[1..])
    }
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IdMemorySemantics(pub u32);
impl IdMemorySemantics {
    pub fn from_raw(data: &[u32]) -> (Self, &[u32]) {
        (Self(data[0]), &data[1..])
    }
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IdScope(pub u32);
impl IdScope {
    pub fn from_raw(data: &[u32]) -> (Self, &[u32]) {
        (Self(data[0]), &data[1..])
    }
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IdRef(pub u32);
impl IdRef {
    pub fn from_raw(data: &[u32]) -> (Self, &[u32]) {
        (Self(data[0]), &data[1..])
    }
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LiteralInteger(pub u32);
impl LiteralInteger {
    pub fn from_raw(data: &[u32]) -> (Self, &[u32]) {
        (Self(data[0]), &data[1..])
    }
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LiteralString(pub String);
impl LiteralString {
    pub fn from_raw(data: &[u32]) -> (Self, &[u32]) {
        let res = parse_string(data);
        (Self(res.0), res.1)
    }
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LiteralContextDependentNumber(pub Vec<u32>);
impl LiteralContextDependentNumber {
    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
        let mut v = vec![];
        while (data.len() > 0) {
            v.push(data[0]);
            data = &data[1..];
        }
        (Self(v), data)
    }
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LiteralExtInstInteger(pub u32);
impl LiteralExtInstInteger {
    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
        (Self(data[0]), &data[1..])
    }
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LiteralSpecConstantOpInteger(pub Vec<u32>);
impl LiteralSpecConstantOpInteger {
    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
        let mut v = vec![];
        while (data.len() > 0) {
            v.push(data[0]);
            data = &data[1..];
        }
        (Self(v), data)
    }
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PairLiteralIntegerIdRef(pub LiteralInteger, pub IdRef);
impl PairLiteralIntegerIdRef {
    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
        (
            Self(
                {
                    let (v, d) = LiteralInteger::from_raw(data);
                    data = d;
                    v
                },
                {
                    let (v, d) = IdRef::from_raw(data);
                    data = d;
                    v
                },
            ),
            data,
        )
    }
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PairIdRefLiteralInteger(pub IdRef, pub LiteralInteger);
impl PairIdRefLiteralInteger {
    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
        (
            Self(
                {
                    let (v, d) = IdRef::from_raw(data);
                    data = d;
                    v
                },
                {
                    let (v, d) = LiteralInteger::from_raw(data);
                    data = d;
                    v
                },
            ),
            data,
        )
    }
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PairIdRefIdRef(pub IdRef, pub IdRef);
impl PairIdRefIdRef {
    pub fn from_raw(mut data: &[u32]) -> (Self, &[u32]) {
        (
            Self(
                {
                    let (v, d) = IdRef::from_raw(data);
                    data = d;
                    v
                },
                {
                    let (v, d) = IdRef::from_raw(data);
                    data = d;
                    v
                },
            ),
            data,
        )
    }
}
fn parse_string(data: &[u32]) -> (String, &[u32]) {
    let bytes = data
        .iter()
        .flat_map(|&n| {
            let b1 = (n & 0xff) as u8;
            let b2 = ((n >> 8) & 0xff) as u8;
            let b3 = ((n >> 16) & 0xff) as u8;
            let b4 = ((n >> 24) & 0xff) as u8;
            vec![b1, b2, b3, b4].into_iter()
        })
        .take_while(|&b| b != 0)
        .collect::<Vec<u8>>();
    let r = 1 + bytes.len() / 4;
    let s = String::from_utf8(bytes).expect("Shader content is not UTF-8");
    (s, &data[r..])
}
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Instruction {
    Nop,
    Undef(IdResultType, IdResult),
    SourceContinued(LiteralString),
    Source(
        SourceLanguage,
        LiteralInteger,
        Option<IdRef>,
        Option<LiteralString>,
    ),
    SourceExtension(LiteralString),
    Name(IdRef, LiteralString),
    MemberName(IdRef, LiteralInteger, LiteralString),
    String(IdResult, LiteralString),
    Line(IdRef, LiteralInteger, LiteralInteger),
    Extension(LiteralString),
    ExtInstImport(IdResult, LiteralString),
    ExtInst(
        IdResultType,
        IdResult,
        IdRef,
        LiteralExtInstInteger,
        Vec<IdRef>,
    ),
    MemoryModel(AddressingModel, MemoryModel),
    EntryPoint(ExecutionModel, IdRef, LiteralString, Vec<IdRef>),
    ExecutionMode(IdRef, ExecutionMode),
    Capability(Capability),
    TypeVoid(IdResult),
    TypeBool(IdResult),
    TypeInt(IdResult, LiteralInteger, LiteralInteger),
    TypeFloat(IdResult, LiteralInteger),
    TypeVector(IdResult, IdRef, LiteralInteger),
    TypeMatrix(IdResult, IdRef, LiteralInteger),
    TypeImage(
        IdResult,
        IdRef,
        Dim,
        LiteralInteger,
        LiteralInteger,
        LiteralInteger,
        LiteralInteger,
        ImageFormat,
        Option<AccessQualifier>,
    ),
    TypeSampler(IdResult),
    TypeSampledImage(IdResult, IdRef),
    TypeArray(IdResult, IdRef, IdRef),
    TypeRuntimeArray(IdResult, IdRef),
    TypeStruct(IdResult, Vec<IdRef>),
    TypeOpaque(IdResult, LiteralString),
    TypePointer(IdResult, StorageClass, IdRef),
    TypeFunction(IdResult, IdRef, Vec<IdRef>),
    TypeEvent(IdResult),
    TypeDeviceEvent(IdResult),
    TypeReserveId(IdResult),
    TypeQueue(IdResult),
    TypePipe(IdResult, AccessQualifier),
    TypeForwardPointer(IdRef, StorageClass),
    ConstantTrue(IdResultType, IdResult),
    ConstantFalse(IdResultType, IdResult),
    Constant(IdResultType, IdResult, LiteralContextDependentNumber),
    ConstantComposite(IdResultType, IdResult, Vec<IdRef>),
    ConstantSampler(
        IdResultType,
        IdResult,
        SamplerAddressingMode,
        LiteralInteger,
        SamplerFilterMode,
    ),
    ConstantNull(IdResultType, IdResult),
    SpecConstantTrue(IdResultType, IdResult),
    SpecConstantFalse(IdResultType, IdResult),
    SpecConstant(IdResultType, IdResult, LiteralContextDependentNumber),
    SpecConstantComposite(IdResultType, IdResult, Vec<IdRef>),
    SpecConstantOp(IdResultType, IdResult, LiteralSpecConstantOpInteger),
    Function(IdResultType, IdResult, FunctionControl, IdRef),
    FunctionParameter(IdResultType, IdResult),
    FunctionEnd,
    FunctionCall(IdResultType, IdResult, IdRef, Vec<IdRef>),
    Variable(IdResultType, IdResult, StorageClass, Option<IdRef>),
    ImageTexelPointer(IdResultType, IdResult, IdRef, IdRef, IdRef),
    Load(IdResultType, IdResult, IdRef, Option<MemoryAccess>),
    Store(IdRef, IdRef, Option<MemoryAccess>),
    CopyMemory(IdRef, IdRef, Option<MemoryAccess>, Option<MemoryAccess>),
    CopyMemorySized(
        IdRef,
        IdRef,
        IdRef,
        Option<MemoryAccess>,
        Option<MemoryAccess>,
    ),
    AccessChain(IdResultType, IdResult, IdRef, Vec<IdRef>),
    InBoundsAccessChain(IdResultType, IdResult, IdRef, Vec<IdRef>),
    PtrAccessChain(IdResultType, IdResult, IdRef, IdRef, Vec<IdRef>),
    ArrayLength(IdResultType, IdResult, IdRef, LiteralInteger),
    GenericPtrMemSemantics(IdResultType, IdResult, IdRef),
    InBoundsPtrAccessChain(IdResultType, IdResult, IdRef, IdRef, Vec<IdRef>),
    Decorate(IdRef, Decoration),
    MemberDecorate(IdRef, LiteralInteger, Decoration),
    DecorationGroup(IdResult),
    GroupDecorate(IdRef, Vec<IdRef>),
    GroupMemberDecorate(IdRef, Vec<PairIdRefLiteralInteger>),
    VectorExtractDynamic(IdResultType, IdResult, IdRef, IdRef),
    VectorInsertDynamic(IdResultType, IdResult, IdRef, IdRef, IdRef),
    VectorShuffle(IdResultType, IdResult, IdRef, IdRef, Vec<LiteralInteger>),
    CompositeConstruct(IdResultType, IdResult, Vec<IdRef>),
    CompositeExtract(IdResultType, IdResult, IdRef, Vec<LiteralInteger>),
    CompositeInsert(IdResultType, IdResult, IdRef, IdRef, Vec<LiteralInteger>),
    CopyObject(IdResultType, IdResult, IdRef),
    Transpose(IdResultType, IdResult, IdRef),
    SampledImage(IdResultType, IdResult, IdRef, IdRef),
    ImageSampleImplicitLod(IdResultType, IdResult, IdRef, IdRef, Option<ImageOperands>),
    ImageSampleExplicitLod(IdResultType, IdResult, IdRef, IdRef, ImageOperands),
    ImageSampleDrefImplicitLod(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        Option<ImageOperands>,
    ),
    ImageSampleDrefExplicitLod(IdResultType, IdResult, IdRef, IdRef, IdRef, ImageOperands),
    ImageSampleProjImplicitLod(IdResultType, IdResult, IdRef, IdRef, Option<ImageOperands>),
    ImageSampleProjExplicitLod(IdResultType, IdResult, IdRef, IdRef, ImageOperands),
    ImageSampleProjDrefImplicitLod(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        Option<ImageOperands>,
    ),
    ImageSampleProjDrefExplicitLod(IdResultType, IdResult, IdRef, IdRef, IdRef, ImageOperands),
    ImageFetch(IdResultType, IdResult, IdRef, IdRef, Option<ImageOperands>),
    ImageGather(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        Option<ImageOperands>,
    ),
    ImageDrefGather(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        Option<ImageOperands>,
    ),
    ImageRead(IdResultType, IdResult, IdRef, IdRef, Option<ImageOperands>),
    ImageWrite(IdRef, IdRef, IdRef, Option<ImageOperands>),
    Image(IdResultType, IdResult, IdRef),
    ImageQueryFormat(IdResultType, IdResult, IdRef),
    ImageQueryOrder(IdResultType, IdResult, IdRef),
    ImageQuerySizeLod(IdResultType, IdResult, IdRef, IdRef),
    ImageQuerySize(IdResultType, IdResult, IdRef),
    ImageQueryLod(IdResultType, IdResult, IdRef, IdRef),
    ImageQueryLevels(IdResultType, IdResult, IdRef),
    ImageQuerySamples(IdResultType, IdResult, IdRef),
    ConvertFToU(IdResultType, IdResult, IdRef),
    ConvertFToS(IdResultType, IdResult, IdRef),
    ConvertSToF(IdResultType, IdResult, IdRef),
    ConvertUToF(IdResultType, IdResult, IdRef),
    UConvert(IdResultType, IdResult, IdRef),
    SConvert(IdResultType, IdResult, IdRef),
    FConvert(IdResultType, IdResult, IdRef),
    QuantizeToF16(IdResultType, IdResult, IdRef),
    ConvertPtrToU(IdResultType, IdResult, IdRef),
    SatConvertSToU(IdResultType, IdResult, IdRef),
    SatConvertUToS(IdResultType, IdResult, IdRef),
    ConvertUToPtr(IdResultType, IdResult, IdRef),
    PtrCastToGeneric(IdResultType, IdResult, IdRef),
    GenericCastToPtr(IdResultType, IdResult, IdRef),
    GenericCastToPtrExplicit(IdResultType, IdResult, IdRef, StorageClass),
    Bitcast(IdResultType, IdResult, IdRef),
    SNegate(IdResultType, IdResult, IdRef),
    FNegate(IdResultType, IdResult, IdRef),
    IAdd(IdResultType, IdResult, IdRef, IdRef),
    FAdd(IdResultType, IdResult, IdRef, IdRef),
    ISub(IdResultType, IdResult, IdRef, IdRef),
    FSub(IdResultType, IdResult, IdRef, IdRef),
    IMul(IdResultType, IdResult, IdRef, IdRef),
    FMul(IdResultType, IdResult, IdRef, IdRef),
    UDiv(IdResultType, IdResult, IdRef, IdRef),
    SDiv(IdResultType, IdResult, IdRef, IdRef),
    FDiv(IdResultType, IdResult, IdRef, IdRef),
    UMod(IdResultType, IdResult, IdRef, IdRef),
    SRem(IdResultType, IdResult, IdRef, IdRef),
    SMod(IdResultType, IdResult, IdRef, IdRef),
    FRem(IdResultType, IdResult, IdRef, IdRef),
    FMod(IdResultType, IdResult, IdRef, IdRef),
    VectorTimesScalar(IdResultType, IdResult, IdRef, IdRef),
    MatrixTimesScalar(IdResultType, IdResult, IdRef, IdRef),
    VectorTimesMatrix(IdResultType, IdResult, IdRef, IdRef),
    MatrixTimesVector(IdResultType, IdResult, IdRef, IdRef),
    MatrixTimesMatrix(IdResultType, IdResult, IdRef, IdRef),
    OuterProduct(IdResultType, IdResult, IdRef, IdRef),
    Dot(IdResultType, IdResult, IdRef, IdRef),
    IAddCarry(IdResultType, IdResult, IdRef, IdRef),
    ISubBorrow(IdResultType, IdResult, IdRef, IdRef),
    UMulExtended(IdResultType, IdResult, IdRef, IdRef),
    SMulExtended(IdResultType, IdResult, IdRef, IdRef),
    Any(IdResultType, IdResult, IdRef),
    All(IdResultType, IdResult, IdRef),
    IsNan(IdResultType, IdResult, IdRef),
    IsInf(IdResultType, IdResult, IdRef),
    IsFinite(IdResultType, IdResult, IdRef),
    IsNormal(IdResultType, IdResult, IdRef),
    SignBitSet(IdResultType, IdResult, IdRef),
    LessOrGreater(IdResultType, IdResult, IdRef, IdRef),
    Ordered(IdResultType, IdResult, IdRef, IdRef),
    Unordered(IdResultType, IdResult, IdRef, IdRef),
    LogicalEqual(IdResultType, IdResult, IdRef, IdRef),
    LogicalNotEqual(IdResultType, IdResult, IdRef, IdRef),
    LogicalOr(IdResultType, IdResult, IdRef, IdRef),
    LogicalAnd(IdResultType, IdResult, IdRef, IdRef),
    LogicalNot(IdResultType, IdResult, IdRef),
    Select(IdResultType, IdResult, IdRef, IdRef, IdRef),
    IEqual(IdResultType, IdResult, IdRef, IdRef),
    INotEqual(IdResultType, IdResult, IdRef, IdRef),
    UGreaterThan(IdResultType, IdResult, IdRef, IdRef),
    SGreaterThan(IdResultType, IdResult, IdRef, IdRef),
    UGreaterThanEqual(IdResultType, IdResult, IdRef, IdRef),
    SGreaterThanEqual(IdResultType, IdResult, IdRef, IdRef),
    ULessThan(IdResultType, IdResult, IdRef, IdRef),
    SLessThan(IdResultType, IdResult, IdRef, IdRef),
    ULessThanEqual(IdResultType, IdResult, IdRef, IdRef),
    SLessThanEqual(IdResultType, IdResult, IdRef, IdRef),
    FOrdEqual(IdResultType, IdResult, IdRef, IdRef),
    FUnordEqual(IdResultType, IdResult, IdRef, IdRef),
    FOrdNotEqual(IdResultType, IdResult, IdRef, IdRef),
    FUnordNotEqual(IdResultType, IdResult, IdRef, IdRef),
    FOrdLessThan(IdResultType, IdResult, IdRef, IdRef),
    FUnordLessThan(IdResultType, IdResult, IdRef, IdRef),
    FOrdGreaterThan(IdResultType, IdResult, IdRef, IdRef),
    FUnordGreaterThan(IdResultType, IdResult, IdRef, IdRef),
    FOrdLessThanEqual(IdResultType, IdResult, IdRef, IdRef),
    FUnordLessThanEqual(IdResultType, IdResult, IdRef, IdRef),
    FOrdGreaterThanEqual(IdResultType, IdResult, IdRef, IdRef),
    FUnordGreaterThanEqual(IdResultType, IdResult, IdRef, IdRef),
    ShiftRightLogical(IdResultType, IdResult, IdRef, IdRef),
    ShiftRightArithmetic(IdResultType, IdResult, IdRef, IdRef),
    ShiftLeftLogical(IdResultType, IdResult, IdRef, IdRef),
    BitwiseOr(IdResultType, IdResult, IdRef, IdRef),
    BitwiseXor(IdResultType, IdResult, IdRef, IdRef),
    BitwiseAnd(IdResultType, IdResult, IdRef, IdRef),
    Not(IdResultType, IdResult, IdRef),
    BitFieldInsert(IdResultType, IdResult, IdRef, IdRef, IdRef, IdRef),
    BitFieldSExtract(IdResultType, IdResult, IdRef, IdRef, IdRef),
    BitFieldUExtract(IdResultType, IdResult, IdRef, IdRef, IdRef),
    BitReverse(IdResultType, IdResult, IdRef),
    BitCount(IdResultType, IdResult, IdRef),
    DPdx(IdResultType, IdResult, IdRef),
    DPdy(IdResultType, IdResult, IdRef),
    Fwidth(IdResultType, IdResult, IdRef),
    DPdxFine(IdResultType, IdResult, IdRef),
    DPdyFine(IdResultType, IdResult, IdRef),
    FwidthFine(IdResultType, IdResult, IdRef),
    DPdxCoarse(IdResultType, IdResult, IdRef),
    DPdyCoarse(IdResultType, IdResult, IdRef),
    FwidthCoarse(IdResultType, IdResult, IdRef),
    EmitVertex,
    EndPrimitive,
    EmitStreamVertex(IdRef),
    EndStreamPrimitive(IdRef),
    ControlBarrier(IdScope, IdScope, IdMemorySemantics),
    MemoryBarrier(IdScope, IdMemorySemantics),
    AtomicLoad(IdResultType, IdResult, IdRef, IdScope, IdMemorySemantics),
    AtomicStore(IdRef, IdScope, IdMemorySemantics, IdRef),
    AtomicExchange(
        IdResultType,
        IdResult,
        IdRef,
        IdScope,
        IdMemorySemantics,
        IdRef,
    ),
    AtomicCompareExchange(
        IdResultType,
        IdResult,
        IdRef,
        IdScope,
        IdMemorySemantics,
        IdMemorySemantics,
        IdRef,
        IdRef,
    ),
    AtomicCompareExchangeWeak(
        IdResultType,
        IdResult,
        IdRef,
        IdScope,
        IdMemorySemantics,
        IdMemorySemantics,
        IdRef,
        IdRef,
    ),
    AtomicIIncrement(IdResultType, IdResult, IdRef, IdScope, IdMemorySemantics),
    AtomicIDecrement(IdResultType, IdResult, IdRef, IdScope, IdMemorySemantics),
    AtomicIAdd(
        IdResultType,
        IdResult,
        IdRef,
        IdScope,
        IdMemorySemantics,
        IdRef,
    ),
    AtomicISub(
        IdResultType,
        IdResult,
        IdRef,
        IdScope,
        IdMemorySemantics,
        IdRef,
    ),
    AtomicSMin(
        IdResultType,
        IdResult,
        IdRef,
        IdScope,
        IdMemorySemantics,
        IdRef,
    ),
    AtomicUMin(
        IdResultType,
        IdResult,
        IdRef,
        IdScope,
        IdMemorySemantics,
        IdRef,
    ),
    AtomicSMax(
        IdResultType,
        IdResult,
        IdRef,
        IdScope,
        IdMemorySemantics,
        IdRef,
    ),
    AtomicUMax(
        IdResultType,
        IdResult,
        IdRef,
        IdScope,
        IdMemorySemantics,
        IdRef,
    ),
    AtomicAnd(
        IdResultType,
        IdResult,
        IdRef,
        IdScope,
        IdMemorySemantics,
        IdRef,
    ),
    AtomicOr(
        IdResultType,
        IdResult,
        IdRef,
        IdScope,
        IdMemorySemantics,
        IdRef,
    ),
    AtomicXor(
        IdResultType,
        IdResult,
        IdRef,
        IdScope,
        IdMemorySemantics,
        IdRef,
    ),
    Phi(IdResultType, IdResult, Vec<PairIdRefIdRef>),
    LoopMerge(IdRef, IdRef, LoopControl),
    SelectionMerge(IdRef, SelectionControl),
    Label(IdResult),
    Branch(IdRef),
    BranchConditional(IdRef, IdRef, IdRef, Vec<LiteralInteger>),
    Switch(IdRef, IdRef, Vec<PairLiteralIntegerIdRef>),
    Kill,
    Return,
    ReturnValue(IdRef),
    Unreachable,
    LifetimeStart(IdRef, LiteralInteger),
    LifetimeStop(IdRef, LiteralInteger),
    GroupAsyncCopy(
        IdResultType,
        IdResult,
        IdScope,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
    ),
    GroupWaitEvents(IdScope, IdRef, IdRef),
    GroupAll(IdResultType, IdResult, IdScope, IdRef),
    GroupAny(IdResultType, IdResult, IdScope, IdRef),
    GroupBroadcast(IdResultType, IdResult, IdScope, IdRef, IdRef),
    GroupIAdd(IdResultType, IdResult, IdScope, GroupOperation, IdRef),
    GroupFAdd(IdResultType, IdResult, IdScope, GroupOperation, IdRef),
    GroupFMin(IdResultType, IdResult, IdScope, GroupOperation, IdRef),
    GroupUMin(IdResultType, IdResult, IdScope, GroupOperation, IdRef),
    GroupSMin(IdResultType, IdResult, IdScope, GroupOperation, IdRef),
    GroupFMax(IdResultType, IdResult, IdScope, GroupOperation, IdRef),
    GroupUMax(IdResultType, IdResult, IdScope, GroupOperation, IdRef),
    GroupSMax(IdResultType, IdResult, IdScope, GroupOperation, IdRef),
    ReadPipe(IdResultType, IdResult, IdRef, IdRef, IdRef, IdRef),
    WritePipe(IdResultType, IdResult, IdRef, IdRef, IdRef, IdRef),
    ReservedReadPipe(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
    ),
    ReservedWritePipe(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
    ),
    ReserveReadPipePackets(IdResultType, IdResult, IdRef, IdRef, IdRef, IdRef),
    ReserveWritePipePackets(IdResultType, IdResult, IdRef, IdRef, IdRef, IdRef),
    CommitReadPipe(IdRef, IdRef, IdRef, IdRef),
    CommitWritePipe(IdRef, IdRef, IdRef, IdRef),
    IsValidReserveId(IdResultType, IdResult, IdRef),
    GetNumPipePackets(IdResultType, IdResult, IdRef, IdRef, IdRef),
    GetMaxPipePackets(IdResultType, IdResult, IdRef, IdRef, IdRef),
    GroupReserveReadPipePackets(IdResultType, IdResult, IdScope, IdRef, IdRef, IdRef, IdRef),
    GroupReserveWritePipePackets(IdResultType, IdResult, IdScope, IdRef, IdRef, IdRef, IdRef),
    GroupCommitReadPipe(IdScope, IdRef, IdRef, IdRef, IdRef),
    GroupCommitWritePipe(IdScope, IdRef, IdRef, IdRef, IdRef),
    EnqueueMarker(IdResultType, IdResult, IdRef, IdRef, IdRef, IdRef),
    EnqueueKernel(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        Vec<IdRef>,
    ),
    GetKernelNDrangeSubGroupCount(IdResultType, IdResult, IdRef, IdRef, IdRef, IdRef, IdRef),
    GetKernelNDrangeMaxSubGroupSize(IdResultType, IdResult, IdRef, IdRef, IdRef, IdRef, IdRef),
    GetKernelWorkGroupSize(IdResultType, IdResult, IdRef, IdRef, IdRef, IdRef),
    GetKernelPreferredWorkGroupSizeMultiple(IdResultType, IdResult, IdRef, IdRef, IdRef, IdRef),
    RetainEvent(IdRef),
    ReleaseEvent(IdRef),
    CreateUserEvent(IdResultType, IdResult),
    IsValidEvent(IdResultType, IdResult, IdRef),
    SetUserEventStatus(IdRef, IdRef),
    CaptureEventProfilingInfo(IdRef, IdRef, IdRef),
    GetDefaultQueue(IdResultType, IdResult),
    BuildNDRange(IdResultType, IdResult, IdRef, IdRef, IdRef),
    ImageSparseSampleImplicitLod(IdResultType, IdResult, IdRef, IdRef, Option<ImageOperands>),
    ImageSparseSampleExplicitLod(IdResultType, IdResult, IdRef, IdRef, ImageOperands),
    ImageSparseSampleDrefImplicitLod(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        Option<ImageOperands>,
    ),
    ImageSparseSampleDrefExplicitLod(IdResultType, IdResult, IdRef, IdRef, IdRef, ImageOperands),
    ImageSparseSampleProjImplicitLod(IdResultType, IdResult, IdRef, IdRef, Option<ImageOperands>),
    ImageSparseSampleProjExplicitLod(IdResultType, IdResult, IdRef, IdRef, ImageOperands),
    ImageSparseSampleProjDrefImplicitLod(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        Option<ImageOperands>,
    ),
    ImageSparseSampleProjDrefExplicitLod(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        ImageOperands,
    ),
    ImageSparseFetch(IdResultType, IdResult, IdRef, IdRef, Option<ImageOperands>),
    ImageSparseGather(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        Option<ImageOperands>,
    ),
    ImageSparseDrefGather(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        Option<ImageOperands>,
    ),
    ImageSparseTexelsResident(IdResultType, IdResult, IdRef),
    NoLine,
    AtomicFlagTestAndSet(IdResultType, IdResult, IdRef, IdScope, IdMemorySemantics),
    AtomicFlagClear(IdRef, IdScope, IdMemorySemantics),
    ImageSparseRead(IdResultType, IdResult, IdRef, IdRef, Option<ImageOperands>),
    SizeOf(IdResultType, IdResult, IdRef),
    TypePipeStorage(IdResult),
    ConstantPipeStorage(
        IdResultType,
        IdResult,
        LiteralInteger,
        LiteralInteger,
        LiteralInteger,
    ),
    CreatePipeFromPipeStorage(IdResultType, IdResult, IdRef),
    GetKernelLocalSizeForSubgroupCount(IdResultType, IdResult, IdRef, IdRef, IdRef, IdRef, IdRef),
    GetKernelMaxNumSubgroups(IdResultType, IdResult, IdRef, IdRef, IdRef, IdRef),
    TypeNamedBarrier(IdResult),
    NamedBarrierInitialize(IdResultType, IdResult, IdRef),
    MemoryNamedBarrier(IdRef, IdScope, IdMemorySemantics),
    ModuleProcessed(LiteralString),
    ExecutionModeId(IdRef, ExecutionMode),
    DecorateId(IdRef, Decoration),
    GroupNonUniformElect(IdResultType, IdResult, IdScope),
    GroupNonUniformAll(IdResultType, IdResult, IdScope, IdRef),
    GroupNonUniformAny(IdResultType, IdResult, IdScope, IdRef),
    GroupNonUniformAllEqual(IdResultType, IdResult, IdScope, IdRef),
    GroupNonUniformBroadcast(IdResultType, IdResult, IdScope, IdRef, IdRef),
    GroupNonUniformBroadcastFirst(IdResultType, IdResult, IdScope, IdRef),
    GroupNonUniformBallot(IdResultType, IdResult, IdScope, IdRef),
    GroupNonUniformInverseBallot(IdResultType, IdResult, IdScope, IdRef),
    GroupNonUniformBallotBitExtract(IdResultType, IdResult, IdScope, IdRef, IdRef),
    GroupNonUniformBallotBitCount(IdResultType, IdResult, IdScope, GroupOperation, IdRef),
    GroupNonUniformBallotFindLSB(IdResultType, IdResult, IdScope, IdRef),
    GroupNonUniformBallotFindMSB(IdResultType, IdResult, IdScope, IdRef),
    GroupNonUniformShuffle(IdResultType, IdResult, IdScope, IdRef, IdRef),
    GroupNonUniformShuffleXor(IdResultType, IdResult, IdScope, IdRef, IdRef),
    GroupNonUniformShuffleUp(IdResultType, IdResult, IdScope, IdRef, IdRef),
    GroupNonUniformShuffleDown(IdResultType, IdResult, IdScope, IdRef, IdRef),
    GroupNonUniformIAdd(
        IdResultType,
        IdResult,
        IdScope,
        GroupOperation,
        IdRef,
        Option<IdRef>,
    ),
    GroupNonUniformFAdd(
        IdResultType,
        IdResult,
        IdScope,
        GroupOperation,
        IdRef,
        Option<IdRef>,
    ),
    GroupNonUniformIMul(
        IdResultType,
        IdResult,
        IdScope,
        GroupOperation,
        IdRef,
        Option<IdRef>,
    ),
    GroupNonUniformFMul(
        IdResultType,
        IdResult,
        IdScope,
        GroupOperation,
        IdRef,
        Option<IdRef>,
    ),
    GroupNonUniformSMin(
        IdResultType,
        IdResult,
        IdScope,
        GroupOperation,
        IdRef,
        Option<IdRef>,
    ),
    GroupNonUniformUMin(
        IdResultType,
        IdResult,
        IdScope,
        GroupOperation,
        IdRef,
        Option<IdRef>,
    ),
    GroupNonUniformFMin(
        IdResultType,
        IdResult,
        IdScope,
        GroupOperation,
        IdRef,
        Option<IdRef>,
    ),
    GroupNonUniformSMax(
        IdResultType,
        IdResult,
        IdScope,
        GroupOperation,
        IdRef,
        Option<IdRef>,
    ),
    GroupNonUniformUMax(
        IdResultType,
        IdResult,
        IdScope,
        GroupOperation,
        IdRef,
        Option<IdRef>,
    ),
    GroupNonUniformFMax(
        IdResultType,
        IdResult,
        IdScope,
        GroupOperation,
        IdRef,
        Option<IdRef>,
    ),
    GroupNonUniformBitwiseAnd(
        IdResultType,
        IdResult,
        IdScope,
        GroupOperation,
        IdRef,
        Option<IdRef>,
    ),
    GroupNonUniformBitwiseOr(
        IdResultType,
        IdResult,
        IdScope,
        GroupOperation,
        IdRef,
        Option<IdRef>,
    ),
    GroupNonUniformBitwiseXor(
        IdResultType,
        IdResult,
        IdScope,
        GroupOperation,
        IdRef,
        Option<IdRef>,
    ),
    GroupNonUniformLogicalAnd(
        IdResultType,
        IdResult,
        IdScope,
        GroupOperation,
        IdRef,
        Option<IdRef>,
    ),
    GroupNonUniformLogicalOr(
        IdResultType,
        IdResult,
        IdScope,
        GroupOperation,
        IdRef,
        Option<IdRef>,
    ),
    GroupNonUniformLogicalXor(
        IdResultType,
        IdResult,
        IdScope,
        GroupOperation,
        IdRef,
        Option<IdRef>,
    ),
    GroupNonUniformQuadBroadcast(IdResultType, IdResult, IdScope, IdRef, IdRef),
    GroupNonUniformQuadSwap(IdResultType, IdResult, IdScope, IdRef, IdRef),
    CopyLogical(IdResultType, IdResult, IdRef),
    PtrEqual(IdResultType, IdResult, IdRef, IdRef),
    PtrNotEqual(IdResultType, IdResult, IdRef, IdRef),
    PtrDiff(IdResultType, IdResult, IdRef, IdRef),
    SubgroupBallotKHR(IdResultType, IdResult, IdRef),
    SubgroupFirstInvocationKHR(IdResultType, IdResult, IdRef),
    SubgroupAllKHR(IdResultType, IdResult, IdRef),
    SubgroupAnyKHR(IdResultType, IdResult, IdRef),
    SubgroupAllEqualKHR(IdResultType, IdResult, IdRef),
    SubgroupReadInvocationKHR(IdResultType, IdResult, IdRef, IdRef),
    GroupIAddNonUniformAMD(IdResultType, IdResult, IdScope, GroupOperation, IdRef),
    GroupFAddNonUniformAMD(IdResultType, IdResult, IdScope, GroupOperation, IdRef),
    GroupFMinNonUniformAMD(IdResultType, IdResult, IdScope, GroupOperation, IdRef),
    GroupUMinNonUniformAMD(IdResultType, IdResult, IdScope, GroupOperation, IdRef),
    GroupSMinNonUniformAMD(IdResultType, IdResult, IdScope, GroupOperation, IdRef),
    GroupFMaxNonUniformAMD(IdResultType, IdResult, IdScope, GroupOperation, IdRef),
    GroupUMaxNonUniformAMD(IdResultType, IdResult, IdScope, GroupOperation, IdRef),
    GroupSMaxNonUniformAMD(IdResultType, IdResult, IdScope, GroupOperation, IdRef),
    FragmentMaskFetchAMD(IdResultType, IdResult, IdRef, IdRef),
    FragmentFetchAMD(IdResultType, IdResult, IdRef, IdRef, IdRef),
    ReadClockKHR(IdResultType, IdResult, IdScope),
    ImageSampleFootprintNV(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        Option<ImageOperands>,
    ),
    GroupNonUniformPartitionNV(IdResultType, IdResult, IdRef),
    WritePackedPrimitiveIndices4x8NV(IdRef, IdRef),
    ReportIntersectionNV(IdResultType, IdResult, IdRef, IdRef),
    IgnoreIntersectionNV,
    TerminateRayNV,
    TraceNV(
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
    ),
    TypeAccelerationStructureNV(IdResult),
    ExecuteCallableNV(IdRef, IdRef),
    TypeCooperativeMatrixNV(IdResult, IdRef, IdScope, IdRef, IdRef),
    CooperativeMatrixLoadNV(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        Option<MemoryAccess>,
    ),
    CooperativeMatrixStoreNV(IdRef, IdRef, IdRef, IdRef, Option<MemoryAccess>),
    CooperativeMatrixMulAddNV(IdResultType, IdResult, IdRef, IdRef, IdRef),
    CooperativeMatrixLengthNV(IdResultType, IdResult, IdRef),
    BeginInvocationInterlockEXT,
    EndInvocationInterlockEXT,
    DemoteToHelperInvocationEXT,
    IsHelperInvocationEXT(IdResultType, IdResult),
    SubgroupShuffleINTEL(IdResultType, IdResult, IdRef, IdRef),
    SubgroupShuffleDownINTEL(IdResultType, IdResult, IdRef, IdRef, IdRef),
    SubgroupShuffleUpINTEL(IdResultType, IdResult, IdRef, IdRef, IdRef),
    SubgroupShuffleXorINTEL(IdResultType, IdResult, IdRef, IdRef),
    SubgroupBlockReadINTEL(IdResultType, IdResult, IdRef),
    SubgroupBlockWriteINTEL(IdRef, IdRef),
    SubgroupImageBlockReadINTEL(IdResultType, IdResult, IdRef, IdRef),
    SubgroupImageBlockWriteINTEL(IdRef, IdRef, IdRef),
    SubgroupImageMediaBlockReadINTEL(IdResultType, IdResult, IdRef, IdRef, IdRef, IdRef),
    SubgroupImageMediaBlockWriteINTEL(IdRef, IdRef, IdRef, IdRef, IdRef),
    UCountLeadingZerosINTEL(IdResultType, IdResult, IdRef),
    UCountTrailingZerosINTEL(IdResultType, IdResult, IdRef),
    AbsISubINTEL(IdResultType, IdResult, IdRef, IdRef),
    AbsUSubINTEL(IdResultType, IdResult, IdRef, IdRef),
    IAddSatINTEL(IdResultType, IdResult, IdRef, IdRef),
    UAddSatINTEL(IdResultType, IdResult, IdRef, IdRef),
    IAverageINTEL(IdResultType, IdResult, IdRef, IdRef),
    UAverageINTEL(IdResultType, IdResult, IdRef, IdRef),
    IAverageRoundedINTEL(IdResultType, IdResult, IdRef, IdRef),
    UAverageRoundedINTEL(IdResultType, IdResult, IdRef, IdRef),
    ISubSatINTEL(IdResultType, IdResult, IdRef, IdRef),
    USubSatINTEL(IdResultType, IdResult, IdRef, IdRef),
    IMul32x16INTEL(IdResultType, IdResult, IdRef, IdRef),
    UMul32x16INTEL(IdResultType, IdResult, IdRef, IdRef),
    DecorateString(IdRef, Decoration),
    DecorateStringGOOGLE(IdRef, Decoration),
    MemberDecorateString(IdRef, LiteralInteger, Decoration),
    MemberDecorateStringGOOGLE(IdRef, LiteralInteger, Decoration),
    VmeImageINTEL(IdResultType, IdResult, IdRef, IdRef),
    TypeVmeImageINTEL(IdResult, IdRef),
    TypeAvcImePayloadINTEL(IdResult),
    TypeAvcRefPayloadINTEL(IdResult),
    TypeAvcSicPayloadINTEL(IdResult),
    TypeAvcMcePayloadINTEL(IdResult),
    TypeAvcMceResultINTEL(IdResult),
    TypeAvcImeResultINTEL(IdResult),
    TypeAvcImeResultSingleReferenceStreamoutINTEL(IdResult),
    TypeAvcImeResultDualReferenceStreamoutINTEL(IdResult),
    TypeAvcImeSingleReferenceStreaminINTEL(IdResult),
    TypeAvcImeDualReferenceStreaminINTEL(IdResult),
    TypeAvcRefResultINTEL(IdResult),
    TypeAvcSicResultINTEL(IdResult),
    SubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
    ),
    SubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL(IdResultType, IdResult, IdRef, IdRef),
    SubgroupAvcMceGetDefaultInterShapePenaltyINTEL(IdResultType, IdResult, IdRef, IdRef),
    SubgroupAvcMceSetInterShapePenaltyINTEL(IdResultType, IdResult, IdRef, IdRef),
    SubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL(IdResultType, IdResult, IdRef, IdRef),
    SubgroupAvcMceSetInterDirectionPenaltyINTEL(IdResultType, IdResult, IdRef, IdRef),
    SubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL(IdResultType, IdResult, IdRef, IdRef),
    SubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL(IdResultType, IdResult, IdRef, IdRef),
    SubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL(IdResultType, IdResult),
    SubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL(IdResultType, IdResult),
    SubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL(IdResultType, IdResult),
    SubgroupAvcMceSetMotionVectorCostFunctionINTEL(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
    ),
    SubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL(IdResultType, IdResult, IdRef, IdRef),
    SubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL(IdResultType, IdResult),
    SubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL(IdResultType, IdResult),
    SubgroupAvcMceSetAcOnlyHaarINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL(IdResultType, IdResult, IdRef, IdRef),
    SubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
    ),
    SubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
    ),
    SubgroupAvcMceConvertToImePayloadINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcMceConvertToImeResultINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcMceConvertToRefPayloadINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcMceConvertToRefResultINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcMceConvertToSicPayloadINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcMceConvertToSicResultINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcMceGetMotionVectorsINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcMceGetInterDistortionsINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcMceGetBestInterDistortionsINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcMceGetInterMajorShapeINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcMceGetInterMinorShapeINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcMceGetInterDirectionsINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcMceGetInterMotionVectorCountINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcMceGetInterReferenceIdsINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
    ),
    SubgroupAvcImeInitializeINTEL(IdResultType, IdResult, IdRef, IdRef, IdRef),
    SubgroupAvcImeSetSingleReferenceINTEL(IdResultType, IdResult, IdRef, IdRef, IdRef),
    SubgroupAvcImeSetDualReferenceINTEL(IdResultType, IdResult, IdRef, IdRef, IdRef, IdRef),
    SubgroupAvcImeRefWindowSizeINTEL(IdResultType, IdResult, IdRef, IdRef),
    SubgroupAvcImeAdjustRefOffsetINTEL(IdResultType, IdResult, IdRef, IdRef, IdRef, IdRef),
    SubgroupAvcImeConvertToMcePayloadINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcImeSetMaxMotionVectorCountINTEL(IdResultType, IdResult, IdRef, IdRef),
    SubgroupAvcImeSetUnidirectionalMixDisableINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcImeSetEarlySearchTerminationThresholdINTEL(IdResultType, IdResult, IdRef, IdRef),
    SubgroupAvcImeSetWeightedSadINTEL(IdResultType, IdResult, IdRef, IdRef),
    SubgroupAvcImeEvaluateWithSingleReferenceINTEL(IdResultType, IdResult, IdRef, IdRef, IdRef),
    SubgroupAvcImeEvaluateWithDualReferenceINTEL(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
    ),
    SubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
    ),
    SubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
    ),
    SubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
    ),
    SubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
    ),
    SubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
    ),
    SubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
    ),
    SubgroupAvcImeConvertToMceResultINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcImeGetSingleReferenceStreaminINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcImeGetDualReferenceStreaminINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcImeStripSingleReferenceStreamoutINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcImeStripDualReferenceStreamoutINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
    ),
    SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
    ),
    SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
    ),
    SubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
    ),
    SubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
    ),
    SubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
    ),
    SubgroupAvcImeGetBorderReachedINTEL(IdResultType, IdResult, IdRef, IdRef),
    SubgroupAvcImeGetTruncatedSearchIndicationINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcFmeInitializeINTEL(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
    ),
    SubgroupAvcBmeInitializeINTEL(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
    ),
    SubgroupAvcRefConvertToMcePayloadINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcRefSetBidirectionalMixDisableINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcRefSetBilinearFilterEnableINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcRefEvaluateWithSingleReferenceINTEL(IdResultType, IdResult, IdRef, IdRef, IdRef),
    SubgroupAvcRefEvaluateWithDualReferenceINTEL(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
    ),
    SubgroupAvcRefEvaluateWithMultiReferenceINTEL(IdResultType, IdResult, IdRef, IdRef, IdRef),
    SubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
    ),
    SubgroupAvcRefConvertToMceResultINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcSicInitializeINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcSicConfigureSkcINTEL(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
    ),
    SubgroupAvcSicConfigureIpeLumaINTEL(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
    ),
    SubgroupAvcSicConfigureIpeLumaChromaINTEL(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
    ),
    SubgroupAvcSicGetMotionVectorMaskINTEL(IdResultType, IdResult, IdRef, IdRef),
    SubgroupAvcSicConvertToMcePayloadINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcSicSetIntraLumaShapePenaltyINTEL(IdResultType, IdResult, IdRef, IdRef),
    SubgroupAvcSicSetIntraLumaModeCostFunctionINTEL(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
    ),
    SubgroupAvcSicSetIntraChromaModeCostFunctionINTEL(IdResultType, IdResult, IdRef, IdRef),
    SubgroupAvcSicSetBilinearFilterEnableINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcSicSetSkcForwardTransformEnableINTEL(IdResultType, IdResult, IdRef, IdRef),
    SubgroupAvcSicSetBlockBasedRawSkipSadINTEL(IdResultType, IdResult, IdRef, IdRef),
    SubgroupAvcSicEvaluateIpeINTEL(IdResultType, IdResult, IdRef, IdRef),
    SubgroupAvcSicEvaluateWithSingleReferenceINTEL(IdResultType, IdResult, IdRef, IdRef, IdRef),
    SubgroupAvcSicEvaluateWithDualReferenceINTEL(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
    ),
    SubgroupAvcSicEvaluateWithMultiReferenceINTEL(IdResultType, IdResult, IdRef, IdRef, IdRef),
    SubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL(
        IdResultType,
        IdResult,
        IdRef,
        IdRef,
        IdRef,
        IdRef,
    ),
    SubgroupAvcSicConvertToMceResultINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcSicGetIpeLumaShapeINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcSicGetBestIpeLumaDistortionINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcSicGetBestIpeChromaDistortionINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcSicGetPackedIpeLumaModesINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcSicGetIpeChromaModeINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL(IdResultType, IdResult, IdRef),
    SubgroupAvcSicGetInterRawSadsINTEL(IdResultType, IdResult, IdRef),
    None(u16),
}
impl Instruction {
    pub fn from_raw(op_code: u16, mut data: &[u32]) -> Self {
        match op_code {
            0u16 => Instruction::Nop,
            1u16 => {
                let instr = Instruction::Undef(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            2u16 => {
                let instr = Instruction::SourceContinued({
                    let (v, d) = LiteralString::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            3u16 => {
                let instr = Instruction::Source(
                    {
                        let (v, d) = SourceLanguage::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralInteger::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = LiteralString::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            4u16 => {
                let instr = Instruction::SourceExtension({
                    let (v, d) = LiteralString::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            5u16 => {
                let instr = Instruction::Name(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralString::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            6u16 => {
                let instr = Instruction::MemberName(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralInteger::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralString::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            7u16 => {
                let instr = Instruction::String(
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralString::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            8u16 => {
                let instr = Instruction::Line(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralInteger::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralInteger::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            10u16 => {
                let instr = Instruction::Extension({
                    let (v, d) = LiteralString::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            11u16 => {
                let instr = Instruction::ExtInstImport(
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralString::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            12u16 => {
                let instr = Instruction::ExtInst(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralExtInstInteger::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let mut v = vec![];
                        while data.len() > 0 {
                            let (s, d) = IdRef::from_raw(data);
                            data = d;
                            v.push(s);
                        }
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            14u16 => {
                let instr = Instruction::MemoryModel(
                    {
                        let (v, d) = AddressingModel::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = MemoryModel::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            15u16 => {
                let instr = Instruction::EntryPoint(
                    {
                        let (v, d) = ExecutionModel::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralString::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let mut v = vec![];
                        while data.len() > 0 {
                            let (s, d) = IdRef::from_raw(data);
                            data = d;
                            v.push(s);
                        }
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            16u16 => {
                let instr = Instruction::ExecutionMode(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = ExecutionMode::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            17u16 => {
                let instr = Instruction::Capability({
                    let (v, d) = Capability::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            19u16 => {
                let instr = Instruction::TypeVoid({
                    let (v, d) = IdResult::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            20u16 => {
                let instr = Instruction::TypeBool({
                    let (v, d) = IdResult::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            21u16 => {
                let instr = Instruction::TypeInt(
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralInteger::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralInteger::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            22u16 => {
                let instr = Instruction::TypeFloat(
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralInteger::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            23u16 => {
                let instr = Instruction::TypeVector(
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralInteger::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            24u16 => {
                let instr = Instruction::TypeMatrix(
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralInteger::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            25u16 => {
                let instr = Instruction::TypeImage(
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = Dim::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralInteger::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralInteger::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralInteger::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralInteger::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = ImageFormat::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = AccessQualifier::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            26u16 => {
                let instr = Instruction::TypeSampler({
                    let (v, d) = IdResult::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            27u16 => {
                let instr = Instruction::TypeSampledImage(
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            28u16 => {
                let instr = Instruction::TypeArray(
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            29u16 => {
                let instr = Instruction::TypeRuntimeArray(
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            30u16 => {
                let instr = Instruction::TypeStruct(
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let mut v = vec![];
                        while data.len() > 0 {
                            let (s, d) = IdRef::from_raw(data);
                            data = d;
                            v.push(s);
                        }
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            31u16 => {
                let instr = Instruction::TypeOpaque(
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralString::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            32u16 => {
                let instr = Instruction::TypePointer(
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = StorageClass::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            33u16 => {
                let instr = Instruction::TypeFunction(
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let mut v = vec![];
                        while data.len() > 0 {
                            let (s, d) = IdRef::from_raw(data);
                            data = d;
                            v.push(s);
                        }
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            34u16 => {
                let instr = Instruction::TypeEvent({
                    let (v, d) = IdResult::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            35u16 => {
                let instr = Instruction::TypeDeviceEvent({
                    let (v, d) = IdResult::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            36u16 => {
                let instr = Instruction::TypeReserveId({
                    let (v, d) = IdResult::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            37u16 => {
                let instr = Instruction::TypeQueue({
                    let (v, d) = IdResult::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            38u16 => {
                let instr = Instruction::TypePipe(
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = AccessQualifier::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            39u16 => {
                let instr = Instruction::TypeForwardPointer(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = StorageClass::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            41u16 => {
                let instr = Instruction::ConstantTrue(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            42u16 => {
                let instr = Instruction::ConstantFalse(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            43u16 => {
                let instr = Instruction::Constant(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralContextDependentNumber::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            44u16 => {
                let instr = Instruction::ConstantComposite(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let mut v = vec![];
                        while data.len() > 0 {
                            let (s, d) = IdRef::from_raw(data);
                            data = d;
                            v.push(s);
                        }
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            45u16 => {
                let instr = Instruction::ConstantSampler(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = SamplerAddressingMode::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralInteger::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = SamplerFilterMode::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            46u16 => {
                let instr = Instruction::ConstantNull(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            48u16 => {
                let instr = Instruction::SpecConstantTrue(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            49u16 => {
                let instr = Instruction::SpecConstantFalse(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            50u16 => {
                let instr = Instruction::SpecConstant(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralContextDependentNumber::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            51u16 => {
                let instr = Instruction::SpecConstantComposite(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let mut v = vec![];
                        while data.len() > 0 {
                            let (s, d) = IdRef::from_raw(data);
                            data = d;
                            v.push(s);
                        }
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            52u16 => {
                let instr = Instruction::SpecConstantOp(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralSpecConstantOpInteger::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            54u16 => {
                let instr = Instruction::Function(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = FunctionControl::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            55u16 => {
                let instr = Instruction::FunctionParameter(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            56u16 => Instruction::FunctionEnd,
            57u16 => {
                let instr = Instruction::FunctionCall(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let mut v = vec![];
                        while data.len() > 0 {
                            let (s, d) = IdRef::from_raw(data);
                            data = d;
                            v.push(s);
                        }
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            59u16 => {
                let instr = Instruction::Variable(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = StorageClass::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            60u16 => {
                let instr = Instruction::ImageTexelPointer(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            61u16 => {
                let instr = Instruction::Load(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = MemoryAccess::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            62u16 => {
                let instr = Instruction::Store(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = MemoryAccess::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            63u16 => {
                let instr = Instruction::CopyMemory(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = MemoryAccess::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = MemoryAccess::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            64u16 => {
                let instr = Instruction::CopyMemorySized(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = MemoryAccess::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = MemoryAccess::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            65u16 => {
                let instr = Instruction::AccessChain(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let mut v = vec![];
                        while data.len() > 0 {
                            let (s, d) = IdRef::from_raw(data);
                            data = d;
                            v.push(s);
                        }
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            66u16 => {
                let instr = Instruction::InBoundsAccessChain(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let mut v = vec![];
                        while data.len() > 0 {
                            let (s, d) = IdRef::from_raw(data);
                            data = d;
                            v.push(s);
                        }
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            67u16 => {
                let instr = Instruction::PtrAccessChain(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let mut v = vec![];
                        while data.len() > 0 {
                            let (s, d) = IdRef::from_raw(data);
                            data = d;
                            v.push(s);
                        }
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            68u16 => {
                let instr = Instruction::ArrayLength(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralInteger::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            69u16 => {
                let instr = Instruction::GenericPtrMemSemantics(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            70u16 => {
                let instr = Instruction::InBoundsPtrAccessChain(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let mut v = vec![];
                        while data.len() > 0 {
                            let (s, d) = IdRef::from_raw(data);
                            data = d;
                            v.push(s);
                        }
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            71u16 => {
                let instr = Instruction::Decorate(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = Decoration::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            72u16 => {
                let instr = Instruction::MemberDecorate(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralInteger::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = Decoration::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            73u16 => {
                let instr = Instruction::DecorationGroup({
                    let (v, d) = IdResult::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            74u16 => {
                let instr = Instruction::GroupDecorate(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let mut v = vec![];
                        while data.len() > 0 {
                            let (s, d) = IdRef::from_raw(data);
                            data = d;
                            v.push(s);
                        }
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            75u16 => {
                let instr = Instruction::GroupMemberDecorate(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let mut v = vec![];
                        while data.len() > 0 {
                            let (s, d) = PairIdRefLiteralInteger::from_raw(data);
                            data = d;
                            v.push(s);
                        }
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            77u16 => {
                let instr = Instruction::VectorExtractDynamic(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            78u16 => {
                let instr = Instruction::VectorInsertDynamic(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            79u16 => {
                let instr = Instruction::VectorShuffle(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let mut v = vec![];
                        while data.len() > 0 {
                            let (s, d) = LiteralInteger::from_raw(data);
                            data = d;
                            v.push(s);
                        }
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            80u16 => {
                let instr = Instruction::CompositeConstruct(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let mut v = vec![];
                        while data.len() > 0 {
                            let (s, d) = IdRef::from_raw(data);
                            data = d;
                            v.push(s);
                        }
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            81u16 => {
                let instr = Instruction::CompositeExtract(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let mut v = vec![];
                        while data.len() > 0 {
                            let (s, d) = LiteralInteger::from_raw(data);
                            data = d;
                            v.push(s);
                        }
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            82u16 => {
                let instr = Instruction::CompositeInsert(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let mut v = vec![];
                        while data.len() > 0 {
                            let (s, d) = LiteralInteger::from_raw(data);
                            data = d;
                            v.push(s);
                        }
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            83u16 => {
                let instr = Instruction::CopyObject(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            84u16 => {
                let instr = Instruction::Transpose(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            86u16 => {
                let instr = Instruction::SampledImage(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            87u16 => {
                let instr = Instruction::ImageSampleImplicitLod(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = ImageOperands::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            88u16 => {
                let instr = Instruction::ImageSampleExplicitLod(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = ImageOperands::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            89u16 => {
                let instr = Instruction::ImageSampleDrefImplicitLod(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = ImageOperands::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            90u16 => {
                let instr = Instruction::ImageSampleDrefExplicitLod(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = ImageOperands::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            91u16 => {
                let instr = Instruction::ImageSampleProjImplicitLod(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = ImageOperands::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            92u16 => {
                let instr = Instruction::ImageSampleProjExplicitLod(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = ImageOperands::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            93u16 => {
                let instr = Instruction::ImageSampleProjDrefImplicitLod(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = ImageOperands::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            94u16 => {
                let instr = Instruction::ImageSampleProjDrefExplicitLod(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = ImageOperands::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            95u16 => {
                let instr = Instruction::ImageFetch(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = ImageOperands::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            96u16 => {
                let instr = Instruction::ImageGather(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = ImageOperands::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            97u16 => {
                let instr = Instruction::ImageDrefGather(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = ImageOperands::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            98u16 => {
                let instr = Instruction::ImageRead(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = ImageOperands::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            99u16 => {
                let instr = Instruction::ImageWrite(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = ImageOperands::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            100u16 => {
                let instr = Instruction::Image(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            101u16 => {
                let instr = Instruction::ImageQueryFormat(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            102u16 => {
                let instr = Instruction::ImageQueryOrder(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            103u16 => {
                let instr = Instruction::ImageQuerySizeLod(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            104u16 => {
                let instr = Instruction::ImageQuerySize(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            105u16 => {
                let instr = Instruction::ImageQueryLod(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            106u16 => {
                let instr = Instruction::ImageQueryLevels(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            107u16 => {
                let instr = Instruction::ImageQuerySamples(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            109u16 => {
                let instr = Instruction::ConvertFToU(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            110u16 => {
                let instr = Instruction::ConvertFToS(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            111u16 => {
                let instr = Instruction::ConvertSToF(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            112u16 => {
                let instr = Instruction::ConvertUToF(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            113u16 => {
                let instr = Instruction::UConvert(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            114u16 => {
                let instr = Instruction::SConvert(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            115u16 => {
                let instr = Instruction::FConvert(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            116u16 => {
                let instr = Instruction::QuantizeToF16(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            117u16 => {
                let instr = Instruction::ConvertPtrToU(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            118u16 => {
                let instr = Instruction::SatConvertSToU(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            119u16 => {
                let instr = Instruction::SatConvertUToS(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            120u16 => {
                let instr = Instruction::ConvertUToPtr(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            121u16 => {
                let instr = Instruction::PtrCastToGeneric(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            122u16 => {
                let instr = Instruction::GenericCastToPtr(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            123u16 => {
                let instr = Instruction::GenericCastToPtrExplicit(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = StorageClass::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            124u16 => {
                let instr = Instruction::Bitcast(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            126u16 => {
                let instr = Instruction::SNegate(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            127u16 => {
                let instr = Instruction::FNegate(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            128u16 => {
                let instr = Instruction::IAdd(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            129u16 => {
                let instr = Instruction::FAdd(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            130u16 => {
                let instr = Instruction::ISub(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            131u16 => {
                let instr = Instruction::FSub(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            132u16 => {
                let instr = Instruction::IMul(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            133u16 => {
                let instr = Instruction::FMul(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            134u16 => {
                let instr = Instruction::UDiv(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            135u16 => {
                let instr = Instruction::SDiv(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            136u16 => {
                let instr = Instruction::FDiv(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            137u16 => {
                let instr = Instruction::UMod(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            138u16 => {
                let instr = Instruction::SRem(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            139u16 => {
                let instr = Instruction::SMod(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            140u16 => {
                let instr = Instruction::FRem(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            141u16 => {
                let instr = Instruction::FMod(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            142u16 => {
                let instr = Instruction::VectorTimesScalar(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            143u16 => {
                let instr = Instruction::MatrixTimesScalar(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            144u16 => {
                let instr = Instruction::VectorTimesMatrix(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            145u16 => {
                let instr = Instruction::MatrixTimesVector(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            146u16 => {
                let instr = Instruction::MatrixTimesMatrix(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            147u16 => {
                let instr = Instruction::OuterProduct(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            148u16 => {
                let instr = Instruction::Dot(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            149u16 => {
                let instr = Instruction::IAddCarry(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            150u16 => {
                let instr = Instruction::ISubBorrow(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            151u16 => {
                let instr = Instruction::UMulExtended(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            152u16 => {
                let instr = Instruction::SMulExtended(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            154u16 => {
                let instr = Instruction::Any(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            155u16 => {
                let instr = Instruction::All(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            156u16 => {
                let instr = Instruction::IsNan(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            157u16 => {
                let instr = Instruction::IsInf(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            158u16 => {
                let instr = Instruction::IsFinite(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            159u16 => {
                let instr = Instruction::IsNormal(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            160u16 => {
                let instr = Instruction::SignBitSet(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            161u16 => {
                let instr = Instruction::LessOrGreater(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            162u16 => {
                let instr = Instruction::Ordered(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            163u16 => {
                let instr = Instruction::Unordered(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            164u16 => {
                let instr = Instruction::LogicalEqual(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            165u16 => {
                let instr = Instruction::LogicalNotEqual(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            166u16 => {
                let instr = Instruction::LogicalOr(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            167u16 => {
                let instr = Instruction::LogicalAnd(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            168u16 => {
                let instr = Instruction::LogicalNot(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            169u16 => {
                let instr = Instruction::Select(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            170u16 => {
                let instr = Instruction::IEqual(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            171u16 => {
                let instr = Instruction::INotEqual(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            172u16 => {
                let instr = Instruction::UGreaterThan(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            173u16 => {
                let instr = Instruction::SGreaterThan(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            174u16 => {
                let instr = Instruction::UGreaterThanEqual(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            175u16 => {
                let instr = Instruction::SGreaterThanEqual(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            176u16 => {
                let instr = Instruction::ULessThan(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            177u16 => {
                let instr = Instruction::SLessThan(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            178u16 => {
                let instr = Instruction::ULessThanEqual(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            179u16 => {
                let instr = Instruction::SLessThanEqual(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            180u16 => {
                let instr = Instruction::FOrdEqual(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            181u16 => {
                let instr = Instruction::FUnordEqual(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            182u16 => {
                let instr = Instruction::FOrdNotEqual(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            183u16 => {
                let instr = Instruction::FUnordNotEqual(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            184u16 => {
                let instr = Instruction::FOrdLessThan(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            185u16 => {
                let instr = Instruction::FUnordLessThan(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            186u16 => {
                let instr = Instruction::FOrdGreaterThan(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            187u16 => {
                let instr = Instruction::FUnordGreaterThan(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            188u16 => {
                let instr = Instruction::FOrdLessThanEqual(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            189u16 => {
                let instr = Instruction::FUnordLessThanEqual(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            190u16 => {
                let instr = Instruction::FOrdGreaterThanEqual(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            191u16 => {
                let instr = Instruction::FUnordGreaterThanEqual(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            194u16 => {
                let instr = Instruction::ShiftRightLogical(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            195u16 => {
                let instr = Instruction::ShiftRightArithmetic(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            196u16 => {
                let instr = Instruction::ShiftLeftLogical(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            197u16 => {
                let instr = Instruction::BitwiseOr(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            198u16 => {
                let instr = Instruction::BitwiseXor(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            199u16 => {
                let instr = Instruction::BitwiseAnd(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            200u16 => {
                let instr = Instruction::Not(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            201u16 => {
                let instr = Instruction::BitFieldInsert(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            202u16 => {
                let instr = Instruction::BitFieldSExtract(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            203u16 => {
                let instr = Instruction::BitFieldUExtract(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            204u16 => {
                let instr = Instruction::BitReverse(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            205u16 => {
                let instr = Instruction::BitCount(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            207u16 => {
                let instr = Instruction::DPdx(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            208u16 => {
                let instr = Instruction::DPdy(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            209u16 => {
                let instr = Instruction::Fwidth(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            210u16 => {
                let instr = Instruction::DPdxFine(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            211u16 => {
                let instr = Instruction::DPdyFine(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            212u16 => {
                let instr = Instruction::FwidthFine(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            213u16 => {
                let instr = Instruction::DPdxCoarse(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            214u16 => {
                let instr = Instruction::DPdyCoarse(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            215u16 => {
                let instr = Instruction::FwidthCoarse(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            218u16 => Instruction::EmitVertex,
            219u16 => Instruction::EndPrimitive,
            220u16 => {
                let instr = Instruction::EmitStreamVertex({
                    let (v, d) = IdRef::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            221u16 => {
                let instr = Instruction::EndStreamPrimitive({
                    let (v, d) = IdRef::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            224u16 => {
                let instr = Instruction::ControlBarrier(
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdMemorySemantics::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            225u16 => {
                let instr = Instruction::MemoryBarrier(
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdMemorySemantics::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            227u16 => {
                let instr = Instruction::AtomicLoad(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdMemorySemantics::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            228u16 => {
                let instr = Instruction::AtomicStore(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdMemorySemantics::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            229u16 => {
                let instr = Instruction::AtomicExchange(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdMemorySemantics::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            230u16 => {
                let instr = Instruction::AtomicCompareExchange(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdMemorySemantics::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdMemorySemantics::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            231u16 => {
                let instr = Instruction::AtomicCompareExchangeWeak(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdMemorySemantics::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdMemorySemantics::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            232u16 => {
                let instr = Instruction::AtomicIIncrement(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdMemorySemantics::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            233u16 => {
                let instr = Instruction::AtomicIDecrement(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdMemorySemantics::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            234u16 => {
                let instr = Instruction::AtomicIAdd(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdMemorySemantics::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            235u16 => {
                let instr = Instruction::AtomicISub(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdMemorySemantics::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            236u16 => {
                let instr = Instruction::AtomicSMin(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdMemorySemantics::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            237u16 => {
                let instr = Instruction::AtomicUMin(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdMemorySemantics::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            238u16 => {
                let instr = Instruction::AtomicSMax(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdMemorySemantics::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            239u16 => {
                let instr = Instruction::AtomicUMax(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdMemorySemantics::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            240u16 => {
                let instr = Instruction::AtomicAnd(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdMemorySemantics::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            241u16 => {
                let instr = Instruction::AtomicOr(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdMemorySemantics::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            242u16 => {
                let instr = Instruction::AtomicXor(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdMemorySemantics::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            245u16 => {
                let instr = Instruction::Phi(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let mut v = vec![];
                        while data.len() > 0 {
                            let (s, d) = PairIdRefIdRef::from_raw(data);
                            data = d;
                            v.push(s);
                        }
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            246u16 => {
                let instr = Instruction::LoopMerge(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LoopControl::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            247u16 => {
                let instr = Instruction::SelectionMerge(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = SelectionControl::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            248u16 => {
                let instr = Instruction::Label({
                    let (v, d) = IdResult::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            249u16 => {
                let instr = Instruction::Branch({
                    let (v, d) = IdRef::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            250u16 => {
                let instr = Instruction::BranchConditional(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let mut v = vec![];
                        while data.len() > 0 {
                            let (s, d) = LiteralInteger::from_raw(data);
                            data = d;
                            v.push(s);
                        }
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            251u16 => {
                let instr = Instruction::Switch(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let mut v = vec![];
                        while data.len() > 0 {
                            let (s, d) = PairLiteralIntegerIdRef::from_raw(data);
                            data = d;
                            v.push(s);
                        }
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            252u16 => Instruction::Kill,
            253u16 => Instruction::Return,
            254u16 => {
                let instr = Instruction::ReturnValue({
                    let (v, d) = IdRef::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            255u16 => Instruction::Unreachable,
            256u16 => {
                let instr = Instruction::LifetimeStart(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralInteger::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            257u16 => {
                let instr = Instruction::LifetimeStop(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralInteger::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            259u16 => {
                let instr = Instruction::GroupAsyncCopy(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            260u16 => {
                let instr = Instruction::GroupWaitEvents(
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            261u16 => {
                let instr = Instruction::GroupAll(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            262u16 => {
                let instr = Instruction::GroupAny(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            263u16 => {
                let instr = Instruction::GroupBroadcast(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            264u16 => {
                let instr = Instruction::GroupIAdd(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            265u16 => {
                let instr = Instruction::GroupFAdd(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            266u16 => {
                let instr = Instruction::GroupFMin(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            267u16 => {
                let instr = Instruction::GroupUMin(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            268u16 => {
                let instr = Instruction::GroupSMin(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            269u16 => {
                let instr = Instruction::GroupFMax(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            270u16 => {
                let instr = Instruction::GroupUMax(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            271u16 => {
                let instr = Instruction::GroupSMax(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            274u16 => {
                let instr = Instruction::ReadPipe(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            275u16 => {
                let instr = Instruction::WritePipe(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            276u16 => {
                let instr = Instruction::ReservedReadPipe(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            277u16 => {
                let instr = Instruction::ReservedWritePipe(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            278u16 => {
                let instr = Instruction::ReserveReadPipePackets(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            279u16 => {
                let instr = Instruction::ReserveWritePipePackets(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            280u16 => {
                let instr = Instruction::CommitReadPipe(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            281u16 => {
                let instr = Instruction::CommitWritePipe(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            282u16 => {
                let instr = Instruction::IsValidReserveId(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            283u16 => {
                let instr = Instruction::GetNumPipePackets(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            284u16 => {
                let instr = Instruction::GetMaxPipePackets(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            285u16 => {
                let instr = Instruction::GroupReserveReadPipePackets(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            286u16 => {
                let instr = Instruction::GroupReserveWritePipePackets(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            287u16 => {
                let instr = Instruction::GroupCommitReadPipe(
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            288u16 => {
                let instr = Instruction::GroupCommitWritePipe(
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            291u16 => {
                let instr = Instruction::EnqueueMarker(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            292u16 => {
                let instr = Instruction::EnqueueKernel(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let mut v = vec![];
                        while data.len() > 0 {
                            let (s, d) = IdRef::from_raw(data);
                            data = d;
                            v.push(s);
                        }
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            293u16 => {
                let instr = Instruction::GetKernelNDrangeSubGroupCount(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            294u16 => {
                let instr = Instruction::GetKernelNDrangeMaxSubGroupSize(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            295u16 => {
                let instr = Instruction::GetKernelWorkGroupSize(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            296u16 => {
                let instr = Instruction::GetKernelPreferredWorkGroupSizeMultiple(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            297u16 => {
                let instr = Instruction::RetainEvent({
                    let (v, d) = IdRef::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            298u16 => {
                let instr = Instruction::ReleaseEvent({
                    let (v, d) = IdRef::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            299u16 => {
                let instr = Instruction::CreateUserEvent(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            300u16 => {
                let instr = Instruction::IsValidEvent(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            301u16 => {
                let instr = Instruction::SetUserEventStatus(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            302u16 => {
                let instr = Instruction::CaptureEventProfilingInfo(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            303u16 => {
                let instr = Instruction::GetDefaultQueue(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            304u16 => {
                let instr = Instruction::BuildNDRange(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            305u16 => {
                let instr = Instruction::ImageSparseSampleImplicitLod(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = ImageOperands::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            306u16 => {
                let instr = Instruction::ImageSparseSampleExplicitLod(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = ImageOperands::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            307u16 => {
                let instr = Instruction::ImageSparseSampleDrefImplicitLod(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = ImageOperands::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            308u16 => {
                let instr = Instruction::ImageSparseSampleDrefExplicitLod(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = ImageOperands::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            309u16 => {
                let instr = Instruction::ImageSparseSampleProjImplicitLod(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = ImageOperands::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            310u16 => {
                let instr = Instruction::ImageSparseSampleProjExplicitLod(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = ImageOperands::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            311u16 => {
                let instr = Instruction::ImageSparseSampleProjDrefImplicitLod(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = ImageOperands::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            312u16 => {
                let instr = Instruction::ImageSparseSampleProjDrefExplicitLod(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = ImageOperands::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            313u16 => {
                let instr = Instruction::ImageSparseFetch(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = ImageOperands::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            314u16 => {
                let instr = Instruction::ImageSparseGather(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = ImageOperands::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            315u16 => {
                let instr = Instruction::ImageSparseDrefGather(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = ImageOperands::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            316u16 => {
                let instr = Instruction::ImageSparseTexelsResident(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            317u16 => Instruction::NoLine,
            318u16 => {
                let instr = Instruction::AtomicFlagTestAndSet(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdMemorySemantics::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            319u16 => {
                let instr = Instruction::AtomicFlagClear(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdMemorySemantics::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            320u16 => {
                let instr = Instruction::ImageSparseRead(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = ImageOperands::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            321u16 => {
                let instr = Instruction::SizeOf(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            322u16 => {
                let instr = Instruction::TypePipeStorage({
                    let (v, d) = IdResult::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            323u16 => {
                let instr = Instruction::ConstantPipeStorage(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralInteger::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralInteger::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralInteger::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            324u16 => {
                let instr = Instruction::CreatePipeFromPipeStorage(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            325u16 => {
                let instr = Instruction::GetKernelLocalSizeForSubgroupCount(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            326u16 => {
                let instr = Instruction::GetKernelMaxNumSubgroups(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            327u16 => {
                let instr = Instruction::TypeNamedBarrier({
                    let (v, d) = IdResult::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            328u16 => {
                let instr = Instruction::NamedBarrierInitialize(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            329u16 => {
                let instr = Instruction::MemoryNamedBarrier(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdMemorySemantics::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            330u16 => {
                let instr = Instruction::ModuleProcessed({
                    let (v, d) = LiteralString::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            331u16 => {
                let instr = Instruction::ExecutionModeId(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = ExecutionMode::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            332u16 => {
                let instr = Instruction::DecorateId(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = Decoration::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            333u16 => {
                let instr = Instruction::GroupNonUniformElect(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            334u16 => {
                let instr = Instruction::GroupNonUniformAll(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            335u16 => {
                let instr = Instruction::GroupNonUniformAny(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            336u16 => {
                let instr = Instruction::GroupNonUniformAllEqual(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            337u16 => {
                let instr = Instruction::GroupNonUniformBroadcast(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            338u16 => {
                let instr = Instruction::GroupNonUniformBroadcastFirst(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            339u16 => {
                let instr = Instruction::GroupNonUniformBallot(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            340u16 => {
                let instr = Instruction::GroupNonUniformInverseBallot(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            341u16 => {
                let instr = Instruction::GroupNonUniformBallotBitExtract(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            342u16 => {
                let instr = Instruction::GroupNonUniformBallotBitCount(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            343u16 => {
                let instr = Instruction::GroupNonUniformBallotFindLSB(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            344u16 => {
                let instr = Instruction::GroupNonUniformBallotFindMSB(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            345u16 => {
                let instr = Instruction::GroupNonUniformShuffle(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            346u16 => {
                let instr = Instruction::GroupNonUniformShuffleXor(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            347u16 => {
                let instr = Instruction::GroupNonUniformShuffleUp(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            348u16 => {
                let instr = Instruction::GroupNonUniformShuffleDown(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            349u16 => {
                let instr = Instruction::GroupNonUniformIAdd(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            350u16 => {
                let instr = Instruction::GroupNonUniformFAdd(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            351u16 => {
                let instr = Instruction::GroupNonUniformIMul(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            352u16 => {
                let instr = Instruction::GroupNonUniformFMul(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            353u16 => {
                let instr = Instruction::GroupNonUniformSMin(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            354u16 => {
                let instr = Instruction::GroupNonUniformUMin(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            355u16 => {
                let instr = Instruction::GroupNonUniformFMin(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            356u16 => {
                let instr = Instruction::GroupNonUniformSMax(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            357u16 => {
                let instr = Instruction::GroupNonUniformUMax(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            358u16 => {
                let instr = Instruction::GroupNonUniformFMax(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            359u16 => {
                let instr = Instruction::GroupNonUniformBitwiseAnd(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            360u16 => {
                let instr = Instruction::GroupNonUniformBitwiseOr(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            361u16 => {
                let instr = Instruction::GroupNonUniformBitwiseXor(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            362u16 => {
                let instr = Instruction::GroupNonUniformLogicalAnd(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            363u16 => {
                let instr = Instruction::GroupNonUniformLogicalOr(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            364u16 => {
                let instr = Instruction::GroupNonUniformLogicalXor(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            365u16 => {
                let instr = Instruction::GroupNonUniformQuadBroadcast(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            366u16 => {
                let instr = Instruction::GroupNonUniformQuadSwap(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            400u16 => {
                let instr = Instruction::CopyLogical(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            401u16 => {
                let instr = Instruction::PtrEqual(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            402u16 => {
                let instr = Instruction::PtrNotEqual(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            403u16 => {
                let instr = Instruction::PtrDiff(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            4421u16 => {
                let instr = Instruction::SubgroupBallotKHR(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            4422u16 => {
                let instr = Instruction::SubgroupFirstInvocationKHR(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            4428u16 => {
                let instr = Instruction::SubgroupAllKHR(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            4429u16 => {
                let instr = Instruction::SubgroupAnyKHR(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            4430u16 => {
                let instr = Instruction::SubgroupAllEqualKHR(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            4432u16 => {
                let instr = Instruction::SubgroupReadInvocationKHR(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5000u16 => {
                let instr = Instruction::GroupIAddNonUniformAMD(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5001u16 => {
                let instr = Instruction::GroupFAddNonUniformAMD(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5002u16 => {
                let instr = Instruction::GroupFMinNonUniformAMD(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5003u16 => {
                let instr = Instruction::GroupUMinNonUniformAMD(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5004u16 => {
                let instr = Instruction::GroupSMinNonUniformAMD(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5005u16 => {
                let instr = Instruction::GroupFMaxNonUniformAMD(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5006u16 => {
                let instr = Instruction::GroupUMaxNonUniformAMD(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5007u16 => {
                let instr = Instruction::GroupSMaxNonUniformAMD(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = GroupOperation::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5011u16 => {
                let instr = Instruction::FragmentMaskFetchAMD(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5012u16 => {
                let instr = Instruction::FragmentFetchAMD(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5056u16 => {
                let instr = Instruction::ReadClockKHR(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5283u16 => {
                let instr = Instruction::ImageSampleFootprintNV(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = ImageOperands::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5296u16 => {
                let instr = Instruction::GroupNonUniformPartitionNV(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5299u16 => {
                let instr = Instruction::WritePackedPrimitiveIndices4x8NV(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5334u16 => {
                let instr = Instruction::ReportIntersectionNV(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5335u16 => Instruction::IgnoreIntersectionNV,
            5336u16 => Instruction::TerminateRayNV,
            5337u16 => {
                let instr = Instruction::TraceNV(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5341u16 => {
                let instr = Instruction::TypeAccelerationStructureNV({
                    let (v, d) = IdResult::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            5344u16 => {
                let instr = Instruction::ExecuteCallableNV(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5358u16 => {
                let instr = Instruction::TypeCooperativeMatrixNV(
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdScope::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5359u16 => {
                let instr = Instruction::CooperativeMatrixLoadNV(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = MemoryAccess::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5360u16 => {
                let instr = Instruction::CooperativeMatrixStoreNV(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        if data.len() > 0 {
                            let (v, d) = MemoryAccess::from_raw(data);
                            data = d;
                            Some(v)
                        } else {
                            None
                        }
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5361u16 => {
                let instr = Instruction::CooperativeMatrixMulAddNV(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5362u16 => {
                let instr = Instruction::CooperativeMatrixLengthNV(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5364u16 => Instruction::BeginInvocationInterlockEXT,
            5365u16 => Instruction::EndInvocationInterlockEXT,
            5380u16 => Instruction::DemoteToHelperInvocationEXT,
            5381u16 => {
                let instr = Instruction::IsHelperInvocationEXT(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5571u16 => {
                let instr = Instruction::SubgroupShuffleINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5572u16 => {
                let instr = Instruction::SubgroupShuffleDownINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5573u16 => {
                let instr = Instruction::SubgroupShuffleUpINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5574u16 => {
                let instr = Instruction::SubgroupShuffleXorINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5575u16 => {
                let instr = Instruction::SubgroupBlockReadINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5576u16 => {
                let instr = Instruction::SubgroupBlockWriteINTEL(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5577u16 => {
                let instr = Instruction::SubgroupImageBlockReadINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5578u16 => {
                let instr = Instruction::SubgroupImageBlockWriteINTEL(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5580u16 => {
                let instr = Instruction::SubgroupImageMediaBlockReadINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5581u16 => {
                let instr = Instruction::SubgroupImageMediaBlockWriteINTEL(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5585u16 => {
                let instr = Instruction::UCountLeadingZerosINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5586u16 => {
                let instr = Instruction::UCountTrailingZerosINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5587u16 => {
                let instr = Instruction::AbsISubINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5588u16 => {
                let instr = Instruction::AbsUSubINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5589u16 => {
                let instr = Instruction::IAddSatINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5590u16 => {
                let instr = Instruction::UAddSatINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5591u16 => {
                let instr = Instruction::IAverageINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5592u16 => {
                let instr = Instruction::UAverageINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5593u16 => {
                let instr = Instruction::IAverageRoundedINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5594u16 => {
                let instr = Instruction::UAverageRoundedINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5595u16 => {
                let instr = Instruction::ISubSatINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5596u16 => {
                let instr = Instruction::USubSatINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5597u16 => {
                let instr = Instruction::IMul32x16INTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5598u16 => {
                let instr = Instruction::UMul32x16INTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5632u16 => {
                let instr = Instruction::DecorateString(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = Decoration::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5632u16 => {
                let instr = Instruction::DecorateStringGOOGLE(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = Decoration::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5633u16 => {
                let instr = Instruction::MemberDecorateString(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralInteger::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = Decoration::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5633u16 => {
                let instr = Instruction::MemberDecorateStringGOOGLE(
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = LiteralInteger::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = Decoration::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5699u16 => {
                let instr = Instruction::VmeImageINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5700u16 => {
                let instr = Instruction::TypeVmeImageINTEL(
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5701u16 => {
                let instr = Instruction::TypeAvcImePayloadINTEL({
                    let (v, d) = IdResult::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            5702u16 => {
                let instr = Instruction::TypeAvcRefPayloadINTEL({
                    let (v, d) = IdResult::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            5703u16 => {
                let instr = Instruction::TypeAvcSicPayloadINTEL({
                    let (v, d) = IdResult::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            5704u16 => {
                let instr = Instruction::TypeAvcMcePayloadINTEL({
                    let (v, d) = IdResult::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            5705u16 => {
                let instr = Instruction::TypeAvcMceResultINTEL({
                    let (v, d) = IdResult::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            5706u16 => {
                let instr = Instruction::TypeAvcImeResultINTEL({
                    let (v, d) = IdResult::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            5707u16 => {
                let instr = Instruction::TypeAvcImeResultSingleReferenceStreamoutINTEL({
                    let (v, d) = IdResult::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            5708u16 => {
                let instr = Instruction::TypeAvcImeResultDualReferenceStreamoutINTEL({
                    let (v, d) = IdResult::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            5709u16 => {
                let instr = Instruction::TypeAvcImeSingleReferenceStreaminINTEL({
                    let (v, d) = IdResult::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            5710u16 => {
                let instr = Instruction::TypeAvcImeDualReferenceStreaminINTEL({
                    let (v, d) = IdResult::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            5711u16 => {
                let instr = Instruction::TypeAvcRefResultINTEL({
                    let (v, d) = IdResult::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            5712u16 => {
                let instr = Instruction::TypeAvcSicResultINTEL({
                    let (v, d) = IdResult::from_raw(data);
                    data = d;
                    v
                });
                assert_eq!(data.len(), 0);
                instr
            }
            5713u16 => {
                let instr =
                    Instruction::SubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL(
                        {
                            let (v, d) = IdResultType::from_raw(data);
                            data = d;
                            v
                        },
                        {
                            let (v, d) = IdResult::from_raw(data);
                            data = d;
                            v
                        },
                        {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            v
                        },
                        {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            v
                        },
                    );
                assert_eq!(data.len(), 0);
                instr
            }
            5714u16 => {
                let instr = Instruction::SubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5715u16 => {
                let instr = Instruction::SubgroupAvcMceGetDefaultInterShapePenaltyINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5716u16 => {
                let instr = Instruction::SubgroupAvcMceSetInterShapePenaltyINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5717u16 => {
                let instr = Instruction::SubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5718u16 => {
                let instr = Instruction::SubgroupAvcMceSetInterDirectionPenaltyINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5719u16 => {
                let instr = Instruction::SubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5720u16 => {
                let instr = Instruction::SubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5721u16 => {
                let instr = Instruction::SubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5722u16 => {
                let instr = Instruction::SubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5723u16 => {
                let instr = Instruction::SubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5724u16 => {
                let instr = Instruction::SubgroupAvcMceSetMotionVectorCostFunctionINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5725u16 => {
                let instr = Instruction::SubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5726u16 => {
                let instr = Instruction::SubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5727u16 => {
                let instr = Instruction::SubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5728u16 => {
                let instr = Instruction::SubgroupAvcMceSetAcOnlyHaarINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5729u16 => {
                let instr = Instruction::SubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5730u16 => {
                let instr =
                    Instruction::SubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL(
                        {
                            let (v, d) = IdResultType::from_raw(data);
                            data = d;
                            v
                        },
                        {
                            let (v, d) = IdResult::from_raw(data);
                            data = d;
                            v
                        },
                        {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            v
                        },
                        {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            v
                        },
                    );
                assert_eq!(data.len(), 0);
                instr
            }
            5731u16 => {
                let instr =
                    Instruction::SubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL(
                        {
                            let (v, d) = IdResultType::from_raw(data);
                            data = d;
                            v
                        },
                        {
                            let (v, d) = IdResult::from_raw(data);
                            data = d;
                            v
                        },
                        {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            v
                        },
                        {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            v
                        },
                        {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            v
                        },
                    );
                assert_eq!(data.len(), 0);
                instr
            }
            5732u16 => {
                let instr = Instruction::SubgroupAvcMceConvertToImePayloadINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5733u16 => {
                let instr = Instruction::SubgroupAvcMceConvertToImeResultINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5734u16 => {
                let instr = Instruction::SubgroupAvcMceConvertToRefPayloadINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5735u16 => {
                let instr = Instruction::SubgroupAvcMceConvertToRefResultINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5736u16 => {
                let instr = Instruction::SubgroupAvcMceConvertToSicPayloadINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5737u16 => {
                let instr = Instruction::SubgroupAvcMceConvertToSicResultINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5738u16 => {
                let instr = Instruction::SubgroupAvcMceGetMotionVectorsINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5739u16 => {
                let instr = Instruction::SubgroupAvcMceGetInterDistortionsINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5740u16 => {
                let instr = Instruction::SubgroupAvcMceGetBestInterDistortionsINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5741u16 => {
                let instr = Instruction::SubgroupAvcMceGetInterMajorShapeINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5742u16 => {
                let instr = Instruction::SubgroupAvcMceGetInterMinorShapeINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5743u16 => {
                let instr = Instruction::SubgroupAvcMceGetInterDirectionsINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5744u16 => {
                let instr = Instruction::SubgroupAvcMceGetInterMotionVectorCountINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5745u16 => {
                let instr = Instruction::SubgroupAvcMceGetInterReferenceIdsINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5746u16 => {
                let instr =
                    Instruction::SubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL(
                        {
                            let (v, d) = IdResultType::from_raw(data);
                            data = d;
                            v
                        },
                        {
                            let (v, d) = IdResult::from_raw(data);
                            data = d;
                            v
                        },
                        {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            v
                        },
                        {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            v
                        },
                        {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            v
                        },
                    );
                assert_eq!(data.len(), 0);
                instr
            }
            5747u16 => {
                let instr = Instruction::SubgroupAvcImeInitializeINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5748u16 => {
                let instr = Instruction::SubgroupAvcImeSetSingleReferenceINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5749u16 => {
                let instr = Instruction::SubgroupAvcImeSetDualReferenceINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5750u16 => {
                let instr = Instruction::SubgroupAvcImeRefWindowSizeINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5751u16 => {
                let instr = Instruction::SubgroupAvcImeAdjustRefOffsetINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5752u16 => {
                let instr = Instruction::SubgroupAvcImeConvertToMcePayloadINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5753u16 => {
                let instr = Instruction::SubgroupAvcImeSetMaxMotionVectorCountINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5754u16 => {
                let instr = Instruction::SubgroupAvcImeSetUnidirectionalMixDisableINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5755u16 => {
                let instr = Instruction::SubgroupAvcImeSetEarlySearchTerminationThresholdINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5756u16 => {
                let instr = Instruction::SubgroupAvcImeSetWeightedSadINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5757u16 => {
                let instr = Instruction::SubgroupAvcImeEvaluateWithSingleReferenceINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5758u16 => {
                let instr = Instruction::SubgroupAvcImeEvaluateWithDualReferenceINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5759u16 => {
                let instr = Instruction::SubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5760u16 => {
                let instr = Instruction::SubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5761u16 => {
                let instr = Instruction::SubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5762u16 => {
                let instr = Instruction::SubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5763u16 => {
                let instr = Instruction::SubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5764u16 => {
                let instr = Instruction::SubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5765u16 => {
                let instr = Instruction::SubgroupAvcImeConvertToMceResultINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5766u16 => {
                let instr = Instruction::SubgroupAvcImeGetSingleReferenceStreaminINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5767u16 => {
                let instr = Instruction::SubgroupAvcImeGetDualReferenceStreaminINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5768u16 => {
                let instr = Instruction::SubgroupAvcImeStripSingleReferenceStreamoutINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5769u16 => {
                let instr = Instruction::SubgroupAvcImeStripDualReferenceStreamoutINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5770u16 => {
                let instr = Instruction :: SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL ( { let ( v , d ) = IdResultType :: from_raw ( data ) ; data = d ; v } , { let ( v , d ) = IdResult :: from_raw ( data ) ; data = d ; v } , { let ( v , d ) = IdRef :: from_raw ( data ) ; data = d ; v } , { let ( v , d ) = IdRef :: from_raw ( data ) ; data = d ; v } , ) ;
                assert_eq!(data.len(), 0);
                instr
            }
            5771u16 => {
                let instr = Instruction :: SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL ( { let ( v , d ) = IdResultType :: from_raw ( data ) ; data = d ; v } , { let ( v , d ) = IdResult :: from_raw ( data ) ; data = d ; v } , { let ( v , d ) = IdRef :: from_raw ( data ) ; data = d ; v } , { let ( v , d ) = IdRef :: from_raw ( data ) ; data = d ; v } , ) ;
                assert_eq!(data.len(), 0);
                instr
            }
            5772u16 => {
                let instr = Instruction :: SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL ( { let ( v , d ) = IdResultType :: from_raw ( data ) ; data = d ; v } , { let ( v , d ) = IdResult :: from_raw ( data ) ; data = d ; v } , { let ( v , d ) = IdRef :: from_raw ( data ) ; data = d ; v } , { let ( v , d ) = IdRef :: from_raw ( data ) ; data = d ; v } , ) ;
                assert_eq!(data.len(), 0);
                instr
            }
            5773u16 => {
                let instr = Instruction :: SubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL ( { let ( v , d ) = IdResultType :: from_raw ( data ) ; data = d ; v } , { let ( v , d ) = IdResult :: from_raw ( data ) ; data = d ; v } , { let ( v , d ) = IdRef :: from_raw ( data ) ; data = d ; v } , { let ( v , d ) = IdRef :: from_raw ( data ) ; data = d ; v } , { let ( v , d ) = IdRef :: from_raw ( data ) ; data = d ; v } , ) ;
                assert_eq!(data.len(), 0);
                instr
            }
            5774u16 => {
                let instr =
                    Instruction::SubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL(
                        {
                            let (v, d) = IdResultType::from_raw(data);
                            data = d;
                            v
                        },
                        {
                            let (v, d) = IdResult::from_raw(data);
                            data = d;
                            v
                        },
                        {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            v
                        },
                        {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            v
                        },
                        {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            v
                        },
                    );
                assert_eq!(data.len(), 0);
                instr
            }
            5775u16 => {
                let instr =
                    Instruction::SubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL(
                        {
                            let (v, d) = IdResultType::from_raw(data);
                            data = d;
                            v
                        },
                        {
                            let (v, d) = IdResult::from_raw(data);
                            data = d;
                            v
                        },
                        {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            v
                        },
                        {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            v
                        },
                        {
                            let (v, d) = IdRef::from_raw(data);
                            data = d;
                            v
                        },
                    );
                assert_eq!(data.len(), 0);
                instr
            }
            5776u16 => {
                let instr = Instruction::SubgroupAvcImeGetBorderReachedINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5777u16 => {
                let instr = Instruction::SubgroupAvcImeGetTruncatedSearchIndicationINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5778u16 => {
                let instr = Instruction::SubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5779u16 => {
                let instr = Instruction::SubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5780u16 => {
                let instr = Instruction::SubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5781u16 => {
                let instr = Instruction::SubgroupAvcFmeInitializeINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5782u16 => {
                let instr = Instruction::SubgroupAvcBmeInitializeINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5783u16 => {
                let instr = Instruction::SubgroupAvcRefConvertToMcePayloadINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5784u16 => {
                let instr = Instruction::SubgroupAvcRefSetBidirectionalMixDisableINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5785u16 => {
                let instr = Instruction::SubgroupAvcRefSetBilinearFilterEnableINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5786u16 => {
                let instr = Instruction::SubgroupAvcRefEvaluateWithSingleReferenceINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5787u16 => {
                let instr = Instruction::SubgroupAvcRefEvaluateWithDualReferenceINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5788u16 => {
                let instr = Instruction::SubgroupAvcRefEvaluateWithMultiReferenceINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5789u16 => {
                let instr = Instruction::SubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5790u16 => {
                let instr = Instruction::SubgroupAvcRefConvertToMceResultINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5791u16 => {
                let instr = Instruction::SubgroupAvcSicInitializeINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5792u16 => {
                let instr = Instruction::SubgroupAvcSicConfigureSkcINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5793u16 => {
                let instr = Instruction::SubgroupAvcSicConfigureIpeLumaINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5794u16 => {
                let instr = Instruction::SubgroupAvcSicConfigureIpeLumaChromaINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5795u16 => {
                let instr = Instruction::SubgroupAvcSicGetMotionVectorMaskINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5796u16 => {
                let instr = Instruction::SubgroupAvcSicConvertToMcePayloadINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5797u16 => {
                let instr = Instruction::SubgroupAvcSicSetIntraLumaShapePenaltyINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5798u16 => {
                let instr = Instruction::SubgroupAvcSicSetIntraLumaModeCostFunctionINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5799u16 => {
                let instr = Instruction::SubgroupAvcSicSetIntraChromaModeCostFunctionINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5800u16 => {
                let instr = Instruction::SubgroupAvcSicSetBilinearFilterEnableINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5801u16 => {
                let instr = Instruction::SubgroupAvcSicSetSkcForwardTransformEnableINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5802u16 => {
                let instr = Instruction::SubgroupAvcSicSetBlockBasedRawSkipSadINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5803u16 => {
                let instr = Instruction::SubgroupAvcSicEvaluateIpeINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5804u16 => {
                let instr = Instruction::SubgroupAvcSicEvaluateWithSingleReferenceINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5805u16 => {
                let instr = Instruction::SubgroupAvcSicEvaluateWithDualReferenceINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5806u16 => {
                let instr = Instruction::SubgroupAvcSicEvaluateWithMultiReferenceINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5807u16 => {
                let instr = Instruction::SubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5808u16 => {
                let instr = Instruction::SubgroupAvcSicConvertToMceResultINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5809u16 => {
                let instr = Instruction::SubgroupAvcSicGetIpeLumaShapeINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5810u16 => {
                let instr = Instruction::SubgroupAvcSicGetBestIpeLumaDistortionINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5811u16 => {
                let instr = Instruction::SubgroupAvcSicGetBestIpeChromaDistortionINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5812u16 => {
                let instr = Instruction::SubgroupAvcSicGetPackedIpeLumaModesINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5813u16 => {
                let instr = Instruction::SubgroupAvcSicGetIpeChromaModeINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5814u16 => {
                let instr = Instruction::SubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5815u16 => {
                let instr = Instruction::SubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            5816u16 => {
                let instr = Instruction::SubgroupAvcSicGetInterRawSadsINTEL(
                    {
                        let (v, d) = IdResultType::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdResult::from_raw(data);
                        data = d;
                        v
                    },
                    {
                        let (v, d) = IdRef::from_raw(data);
                        data = d;
                        v
                    },
                );
                assert_eq!(data.len(), 0);
                instr
            }
            _ => Instruction::None(op_code),
        }
    }
}
impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Nop => {
                write!(f, "Instruction::Nop")?;
            }
            Instruction::Undef(x_0, x_1) => {
                write!(f, "%{1:?} = Instruction::Undef {0:?}", x_0, x_1,)?;
            }
            Instruction::SourceContinued(x_0) => {
                write!(f, "Instruction::SourceContinued {0:?}", x_0,)?;
            }
            Instruction::Source(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "Instruction::Source {0:?} {1:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::SourceExtension(x_0) => {
                write!(f, "Instruction::SourceExtension {0:?}", x_0,)?;
            }
            Instruction::Name(x_0, x_1) => {
                write!(f, "Instruction::Name {0:?} {1:?}", x_0, x_1,)?;
            }
            Instruction::MemberName(x_0, x_1, x_2) => {
                write!(
                    f,
                    "Instruction::MemberName {0:?} {1:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::String(x_0, x_1) => {
                write!(f, "%{0:?} = Instruction::String {1:?}", x_0, x_1,)?;
            }
            Instruction::Line(x_0, x_1, x_2) => {
                write!(f, "Instruction::Line {0:?} {1:?} {2:?}", x_0, x_1, x_2,)?;
            }
            Instruction::Extension(x_0) => {
                write!(f, "Instruction::Extension {0:?}", x_0,)?;
            }
            Instruction::ExtInstImport(x_0, x_1) => {
                write!(f, "%{0:?} = Instruction::ExtInstImport {1:?}", x_0, x_1,)?;
            }
            Instruction::ExtInst(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ExtInst {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::MemoryModel(x_0, x_1) => {
                write!(f, "Instruction::MemoryModel {0:?} {1:?}", x_0, x_1,)?;
            }
            Instruction::EntryPoint(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "Instruction::EntryPoint {0:?} {1:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::ExecutionMode(x_0, x_1) => {
                write!(f, "Instruction::ExecutionMode {0:?} {1:?}", x_0, x_1,)?;
            }
            Instruction::Capability(x_0) => {
                write!(f, "Instruction::Capability {0:?}", x_0,)?;
            }
            Instruction::TypeVoid(x_0) => {
                write!(f, "%{0:?} = Instruction::TypeVoid ", x_0,)?;
            }
            Instruction::TypeBool(x_0) => {
                write!(f, "%{0:?} = Instruction::TypeBool ", x_0,)?;
            }
            Instruction::TypeInt(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{0:?} = Instruction::TypeInt {1:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::TypeFloat(x_0, x_1) => {
                write!(f, "%{0:?} = Instruction::TypeFloat {1:?}", x_0, x_1,)?;
            }
            Instruction::TypeVector(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{0:?} = Instruction::TypeVector {1:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::TypeMatrix(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{0:?} = Instruction::TypeMatrix {1:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::TypeImage(x_0, x_1, x_2, x_3, x_4, x_5, x_6, x_7, x_8) => {
                write ! ( f , "%{0:?} = Instruction::TypeImage {1:?} {2:?} {3:?} {4:?} {5:?} {6:?} {7:?} {8:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , x_6 , x_7 , x_8 , ) ? ;
            }
            Instruction::TypeSampler(x_0) => {
                write!(f, "%{0:?} = Instruction::TypeSampler ", x_0,)?;
            }
            Instruction::TypeSampledImage(x_0, x_1) => {
                write!(f, "%{0:?} = Instruction::TypeSampledImage {1:?}", x_0, x_1,)?;
            }
            Instruction::TypeArray(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{0:?} = Instruction::TypeArray {1:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::TypeRuntimeArray(x_0, x_1) => {
                write!(f, "%{0:?} = Instruction::TypeRuntimeArray {1:?}", x_0, x_1,)?;
            }
            Instruction::TypeStruct(x_0, x_1) => {
                write!(f, "%{0:?} = Instruction::TypeStruct {1:?}", x_0, x_1,)?;
            }
            Instruction::TypeOpaque(x_0, x_1) => {
                write!(f, "%{0:?} = Instruction::TypeOpaque {1:?}", x_0, x_1,)?;
            }
            Instruction::TypePointer(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{0:?} = Instruction::TypePointer {1:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::TypeFunction(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{0:?} = Instruction::TypeFunction {1:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::TypeEvent(x_0) => {
                write!(f, "%{0:?} = Instruction::TypeEvent ", x_0,)?;
            }
            Instruction::TypeDeviceEvent(x_0) => {
                write!(f, "%{0:?} = Instruction::TypeDeviceEvent ", x_0,)?;
            }
            Instruction::TypeReserveId(x_0) => {
                write!(f, "%{0:?} = Instruction::TypeReserveId ", x_0,)?;
            }
            Instruction::TypeQueue(x_0) => {
                write!(f, "%{0:?} = Instruction::TypeQueue ", x_0,)?;
            }
            Instruction::TypePipe(x_0, x_1) => {
                write!(f, "%{0:?} = Instruction::TypePipe {1:?}", x_0, x_1,)?;
            }
            Instruction::TypeForwardPointer(x_0, x_1) => {
                write!(f, "Instruction::TypeForwardPointer {0:?} {1:?}", x_0, x_1,)?;
            }
            Instruction::ConstantTrue(x_0, x_1) => {
                write!(f, "%{1:?} = Instruction::ConstantTrue {0:?}", x_0, x_1,)?;
            }
            Instruction::ConstantFalse(x_0, x_1) => {
                write!(f, "%{1:?} = Instruction::ConstantFalse {0:?}", x_0, x_1,)?;
            }
            Instruction::Constant(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::Constant {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::ConstantComposite(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ConstantComposite {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::ConstantSampler(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ConstantSampler {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::ConstantNull(x_0, x_1) => {
                write!(f, "%{1:?} = Instruction::ConstantNull {0:?}", x_0, x_1,)?;
            }
            Instruction::SpecConstantTrue(x_0, x_1) => {
                write!(f, "%{1:?} = Instruction::SpecConstantTrue {0:?}", x_0, x_1,)?;
            }
            Instruction::SpecConstantFalse(x_0, x_1) => {
                write!(f, "%{1:?} = Instruction::SpecConstantFalse {0:?}", x_0, x_1,)?;
            }
            Instruction::SpecConstant(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SpecConstant {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SpecConstantComposite(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SpecConstantComposite {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SpecConstantOp(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SpecConstantOp {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::Function(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::Function {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::FunctionParameter(x_0, x_1) => {
                write!(f, "%{1:?} = Instruction::FunctionParameter {0:?}", x_0, x_1,)?;
            }
            Instruction::FunctionEnd => {
                write!(f, "Instruction::FunctionEnd")?;
            }
            Instruction::FunctionCall(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::FunctionCall {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::Variable(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::Variable {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::ImageTexelPointer(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ImageTexelPointer {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::Load(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::Load {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::Store(x_0, x_1, x_2) => {
                write!(f, "Instruction::Store {0:?} {1:?} {2:?}", x_0, x_1, x_2,)?;
            }
            Instruction::CopyMemory(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "Instruction::CopyMemory {0:?} {1:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::CopyMemorySized(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "Instruction::CopyMemorySized {0:?} {1:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::AccessChain(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::AccessChain {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::InBoundsAccessChain(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::InBoundsAccessChain {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::PtrAccessChain(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::PtrAccessChain {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::ArrayLength(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ArrayLength {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::GenericPtrMemSemantics(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GenericPtrMemSemantics {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::InBoundsPtrAccessChain(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::InBoundsPtrAccessChain {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::Decorate(x_0, x_1) => {
                write!(f, "Instruction::Decorate {0:?} {1:?}", x_0, x_1,)?;
            }
            Instruction::MemberDecorate(x_0, x_1, x_2) => {
                write!(
                    f,
                    "Instruction::MemberDecorate {0:?} {1:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::DecorationGroup(x_0) => {
                write!(f, "%{0:?} = Instruction::DecorationGroup ", x_0,)?;
            }
            Instruction::GroupDecorate(x_0, x_1) => {
                write!(f, "Instruction::GroupDecorate {0:?} {1:?}", x_0, x_1,)?;
            }
            Instruction::GroupMemberDecorate(x_0, x_1) => {
                write!(f, "Instruction::GroupMemberDecorate {0:?} {1:?}", x_0, x_1,)?;
            }
            Instruction::VectorExtractDynamic(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::VectorExtractDynamic {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::VectorInsertDynamic(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::VectorInsertDynamic {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::VectorShuffle(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::VectorShuffle {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::CompositeConstruct(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::CompositeConstruct {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::CompositeExtract(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::CompositeExtract {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::CompositeInsert(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::CompositeInsert {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::CopyObject(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::CopyObject {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::Transpose(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::Transpose {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SampledImage(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SampledImage {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::ImageSampleImplicitLod(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ImageSampleImplicitLod {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::ImageSampleExplicitLod(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ImageSampleExplicitLod {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::ImageSampleDrefImplicitLod(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write ! ( f , "%{1:?} = Instruction::ImageSampleDrefImplicitLod {0:?} {2:?} {3:?} {4:?} {5:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , ) ? ;
            }
            Instruction::ImageSampleDrefExplicitLod(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write ! ( f , "%{1:?} = Instruction::ImageSampleDrefExplicitLod {0:?} {2:?} {3:?} {4:?} {5:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , ) ? ;
            }
            Instruction::ImageSampleProjImplicitLod(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ImageSampleProjImplicitLod {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::ImageSampleProjExplicitLod(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ImageSampleProjExplicitLod {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::ImageSampleProjDrefImplicitLod(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write ! ( f , "%{1:?} = Instruction::ImageSampleProjDrefImplicitLod {0:?} {2:?} {3:?} {4:?} {5:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , ) ? ;
            }
            Instruction::ImageSampleProjDrefExplicitLod(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write ! ( f , "%{1:?} = Instruction::ImageSampleProjDrefExplicitLod {0:?} {2:?} {3:?} {4:?} {5:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , ) ? ;
            }
            Instruction::ImageFetch(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ImageFetch {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::ImageGather(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ImageGather {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::ImageDrefGather(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ImageDrefGather {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::ImageRead(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ImageRead {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::ImageWrite(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "Instruction::ImageWrite {0:?} {1:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::Image(x_0, x_1, x_2) => {
                write!(f, "%{1:?} = Instruction::Image {0:?} {2:?}", x_0, x_1, x_2,)?;
            }
            Instruction::ImageQueryFormat(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ImageQueryFormat {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::ImageQueryOrder(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ImageQueryOrder {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::ImageQuerySizeLod(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ImageQuerySizeLod {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::ImageQuerySize(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ImageQuerySize {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::ImageQueryLod(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ImageQueryLod {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::ImageQueryLevels(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ImageQueryLevels {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::ImageQuerySamples(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ImageQuerySamples {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::ConvertFToU(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ConvertFToU {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::ConvertFToS(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ConvertFToS {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::ConvertSToF(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ConvertSToF {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::ConvertUToF(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ConvertUToF {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::UConvert(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::UConvert {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SConvert(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SConvert {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::FConvert(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::FConvert {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::QuantizeToF16(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::QuantizeToF16 {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::ConvertPtrToU(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ConvertPtrToU {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SatConvertSToU(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SatConvertSToU {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SatConvertUToS(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SatConvertUToS {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::ConvertUToPtr(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ConvertUToPtr {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::PtrCastToGeneric(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::PtrCastToGeneric {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::GenericCastToPtr(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GenericCastToPtr {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::GenericCastToPtrExplicit(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GenericCastToPtrExplicit {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::Bitcast(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::Bitcast {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SNegate(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SNegate {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::FNegate(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::FNegate {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::IAdd(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::IAdd {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::FAdd(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::FAdd {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::ISub(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ISub {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::FSub(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::FSub {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::IMul(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::IMul {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::FMul(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::FMul {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::UDiv(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::UDiv {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::SDiv(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SDiv {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::FDiv(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::FDiv {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::UMod(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::UMod {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::SRem(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SRem {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::SMod(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SMod {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::FRem(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::FRem {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::FMod(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::FMod {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::VectorTimesScalar(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::VectorTimesScalar {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::MatrixTimesScalar(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::MatrixTimesScalar {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::VectorTimesMatrix(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::VectorTimesMatrix {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::MatrixTimesVector(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::MatrixTimesVector {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::MatrixTimesMatrix(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::MatrixTimesMatrix {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::OuterProduct(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::OuterProduct {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::Dot(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::Dot {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::IAddCarry(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::IAddCarry {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::ISubBorrow(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ISubBorrow {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::UMulExtended(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::UMulExtended {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::SMulExtended(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SMulExtended {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::Any(x_0, x_1, x_2) => {
                write!(f, "%{1:?} = Instruction::Any {0:?} {2:?}", x_0, x_1, x_2,)?;
            }
            Instruction::All(x_0, x_1, x_2) => {
                write!(f, "%{1:?} = Instruction::All {0:?} {2:?}", x_0, x_1, x_2,)?;
            }
            Instruction::IsNan(x_0, x_1, x_2) => {
                write!(f, "%{1:?} = Instruction::IsNan {0:?} {2:?}", x_0, x_1, x_2,)?;
            }
            Instruction::IsInf(x_0, x_1, x_2) => {
                write!(f, "%{1:?} = Instruction::IsInf {0:?} {2:?}", x_0, x_1, x_2,)?;
            }
            Instruction::IsFinite(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::IsFinite {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::IsNormal(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::IsNormal {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SignBitSet(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SignBitSet {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::LessOrGreater(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::LessOrGreater {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::Ordered(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::Ordered {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::Unordered(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::Unordered {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::LogicalEqual(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::LogicalEqual {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::LogicalNotEqual(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::LogicalNotEqual {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::LogicalOr(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::LogicalOr {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::LogicalAnd(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::LogicalAnd {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::LogicalNot(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::LogicalNot {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::Select(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::Select {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::IEqual(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::IEqual {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::INotEqual(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::INotEqual {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::UGreaterThan(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::UGreaterThan {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::SGreaterThan(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SGreaterThan {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::UGreaterThanEqual(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::UGreaterThanEqual {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::SGreaterThanEqual(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SGreaterThanEqual {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::ULessThan(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ULessThan {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::SLessThan(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SLessThan {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::ULessThanEqual(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ULessThanEqual {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::SLessThanEqual(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SLessThanEqual {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::FOrdEqual(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::FOrdEqual {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::FUnordEqual(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::FUnordEqual {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::FOrdNotEqual(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::FOrdNotEqual {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::FUnordNotEqual(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::FUnordNotEqual {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::FOrdLessThan(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::FOrdLessThan {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::FUnordLessThan(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::FUnordLessThan {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::FOrdGreaterThan(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::FOrdGreaterThan {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::FUnordGreaterThan(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::FUnordGreaterThan {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::FOrdLessThanEqual(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::FOrdLessThanEqual {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::FUnordLessThanEqual(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::FUnordLessThanEqual {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::FOrdGreaterThanEqual(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::FOrdGreaterThanEqual {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::FUnordGreaterThanEqual(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::FUnordGreaterThanEqual {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::ShiftRightLogical(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ShiftRightLogical {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::ShiftRightArithmetic(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ShiftRightArithmetic {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::ShiftLeftLogical(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ShiftLeftLogical {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::BitwiseOr(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::BitwiseOr {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::BitwiseXor(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::BitwiseXor {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::BitwiseAnd(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::BitwiseAnd {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::Not(x_0, x_1, x_2) => {
                write!(f, "%{1:?} = Instruction::Not {0:?} {2:?}", x_0, x_1, x_2,)?;
            }
            Instruction::BitFieldInsert(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::BitFieldInsert {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::BitFieldSExtract(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::BitFieldSExtract {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::BitFieldUExtract(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::BitFieldUExtract {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::BitReverse(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::BitReverse {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::BitCount(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::BitCount {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::DPdx(x_0, x_1, x_2) => {
                write!(f, "%{1:?} = Instruction::DPdx {0:?} {2:?}", x_0, x_1, x_2,)?;
            }
            Instruction::DPdy(x_0, x_1, x_2) => {
                write!(f, "%{1:?} = Instruction::DPdy {0:?} {2:?}", x_0, x_1, x_2,)?;
            }
            Instruction::Fwidth(x_0, x_1, x_2) => {
                write!(f, "%{1:?} = Instruction::Fwidth {0:?} {2:?}", x_0, x_1, x_2,)?;
            }
            Instruction::DPdxFine(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::DPdxFine {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::DPdyFine(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::DPdyFine {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::FwidthFine(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::FwidthFine {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::DPdxCoarse(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::DPdxCoarse {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::DPdyCoarse(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::DPdyCoarse {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::FwidthCoarse(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::FwidthCoarse {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::EmitVertex => {
                write!(f, "Instruction::EmitVertex")?;
            }
            Instruction::EndPrimitive => {
                write!(f, "Instruction::EndPrimitive")?;
            }
            Instruction::EmitStreamVertex(x_0) => {
                write!(f, "Instruction::EmitStreamVertex {0:?}", x_0,)?;
            }
            Instruction::EndStreamPrimitive(x_0) => {
                write!(f, "Instruction::EndStreamPrimitive {0:?}", x_0,)?;
            }
            Instruction::ControlBarrier(x_0, x_1, x_2) => {
                write!(
                    f,
                    "Instruction::ControlBarrier {0:?} {1:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::MemoryBarrier(x_0, x_1) => {
                write!(f, "Instruction::MemoryBarrier {0:?} {1:?}", x_0, x_1,)?;
            }
            Instruction::AtomicLoad(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::AtomicLoad {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::AtomicStore(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "Instruction::AtomicStore {0:?} {1:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::AtomicExchange(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::AtomicExchange {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::AtomicCompareExchange(x_0, x_1, x_2, x_3, x_4, x_5, x_6, x_7) => {
                write ! ( f , "%{1:?} = Instruction::AtomicCompareExchange {0:?} {2:?} {3:?} {4:?} {5:?} {6:?} {7:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , x_6 , x_7 , ) ? ;
            }
            Instruction::AtomicCompareExchangeWeak(x_0, x_1, x_2, x_3, x_4, x_5, x_6, x_7) => {
                write ! ( f , "%{1:?} = Instruction::AtomicCompareExchangeWeak {0:?} {2:?} {3:?} {4:?} {5:?} {6:?} {7:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , x_6 , x_7 , ) ? ;
            }
            Instruction::AtomicIIncrement(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::AtomicIIncrement {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::AtomicIDecrement(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::AtomicIDecrement {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::AtomicIAdd(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::AtomicIAdd {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::AtomicISub(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::AtomicISub {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::AtomicSMin(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::AtomicSMin {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::AtomicUMin(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::AtomicUMin {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::AtomicSMax(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::AtomicSMax {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::AtomicUMax(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::AtomicUMax {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::AtomicAnd(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::AtomicAnd {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::AtomicOr(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::AtomicOr {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::AtomicXor(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::AtomicXor {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::Phi(x_0, x_1, x_2) => {
                write!(f, "%{1:?} = Instruction::Phi {0:?} {2:?}", x_0, x_1, x_2,)?;
            }
            Instruction::LoopMerge(x_0, x_1, x_2) => {
                write!(f, "Instruction::LoopMerge {0:?} {1:?} {2:?}", x_0, x_1, x_2,)?;
            }
            Instruction::SelectionMerge(x_0, x_1) => {
                write!(f, "Instruction::SelectionMerge {0:?} {1:?}", x_0, x_1,)?;
            }
            Instruction::Label(x_0) => {
                write!(f, "%{0:?} = Instruction::Label ", x_0,)?;
            }
            Instruction::Branch(x_0) => {
                write!(f, "Instruction::Branch {0:?}", x_0,)?;
            }
            Instruction::BranchConditional(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "Instruction::BranchConditional {0:?} {1:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::Switch(x_0, x_1, x_2) => {
                write!(f, "Instruction::Switch {0:?} {1:?} {2:?}", x_0, x_1, x_2,)?;
            }
            Instruction::Kill => {
                write!(f, "Instruction::Kill")?;
            }
            Instruction::Return => {
                write!(f, "Instruction::Return")?;
            }
            Instruction::ReturnValue(x_0) => {
                write!(f, "Instruction::ReturnValue {0:?}", x_0,)?;
            }
            Instruction::Unreachable => {
                write!(f, "Instruction::Unreachable")?;
            }
            Instruction::LifetimeStart(x_0, x_1) => {
                write!(f, "Instruction::LifetimeStart {0:?} {1:?}", x_0, x_1,)?;
            }
            Instruction::LifetimeStop(x_0, x_1) => {
                write!(f, "Instruction::LifetimeStop {0:?} {1:?}", x_0, x_1,)?;
            }
            Instruction::GroupAsyncCopy(x_0, x_1, x_2, x_3, x_4, x_5, x_6, x_7) => {
                write ! ( f , "%{1:?} = Instruction::GroupAsyncCopy {0:?} {2:?} {3:?} {4:?} {5:?} {6:?} {7:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , x_6 , x_7 , ) ? ;
            }
            Instruction::GroupWaitEvents(x_0, x_1, x_2) => {
                write!(
                    f,
                    "Instruction::GroupWaitEvents {0:?} {1:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::GroupAll(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupAll {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::GroupAny(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupAny {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::GroupBroadcast(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupBroadcast {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::GroupIAdd(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupIAdd {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::GroupFAdd(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupFAdd {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::GroupFMin(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupFMin {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::GroupUMin(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupUMin {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::GroupSMin(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupSMin {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::GroupFMax(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupFMax {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::GroupUMax(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupUMax {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::GroupSMax(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupSMax {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::ReadPipe(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ReadPipe {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::WritePipe(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::WritePipe {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::ReservedReadPipe(x_0, x_1, x_2, x_3, x_4, x_5, x_6, x_7) => {
                write ! ( f , "%{1:?} = Instruction::ReservedReadPipe {0:?} {2:?} {3:?} {4:?} {5:?} {6:?} {7:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , x_6 , x_7 , ) ? ;
            }
            Instruction::ReservedWritePipe(x_0, x_1, x_2, x_3, x_4, x_5, x_6, x_7) => {
                write ! ( f , "%{1:?} = Instruction::ReservedWritePipe {0:?} {2:?} {3:?} {4:?} {5:?} {6:?} {7:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , x_6 , x_7 , ) ? ;
            }
            Instruction::ReserveReadPipePackets(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ReserveReadPipePackets {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::ReserveWritePipePackets(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ReserveWritePipePackets {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::CommitReadPipe(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "Instruction::CommitReadPipe {0:?} {1:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::CommitWritePipe(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "Instruction::CommitWritePipe {0:?} {1:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::IsValidReserveId(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::IsValidReserveId {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::GetNumPipePackets(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GetNumPipePackets {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::GetMaxPipePackets(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GetMaxPipePackets {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::GroupReserveReadPipePackets(x_0, x_1, x_2, x_3, x_4, x_5, x_6) => {
                write ! ( f , "%{1:?} = Instruction::GroupReserveReadPipePackets {0:?} {2:?} {3:?} {4:?} {5:?} {6:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , x_6 , ) ? ;
            }
            Instruction::GroupReserveWritePipePackets(x_0, x_1, x_2, x_3, x_4, x_5, x_6) => {
                write ! ( f , "%{1:?} = Instruction::GroupReserveWritePipePackets {0:?} {2:?} {3:?} {4:?} {5:?} {6:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , x_6 , ) ? ;
            }
            Instruction::GroupCommitReadPipe(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "Instruction::GroupCommitReadPipe {0:?} {1:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::GroupCommitWritePipe(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "Instruction::GroupCommitWritePipe {0:?} {1:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::EnqueueMarker(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::EnqueueMarker {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::EnqueueKernel(
                x_0,
                x_1,
                x_2,
                x_3,
                x_4,
                x_5,
                x_6,
                x_7,
                x_8,
                x_9,
                x_10,
                x_11,
                x_12,
            ) => {
                write ! ( f , "%{1:?} = Instruction::EnqueueKernel {0:?} {2:?} {3:?} {4:?} {5:?} {6:?} {7:?} {8:?} {9:?} {10:?} {11:?} {12:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , x_6 , x_7 , x_8 , x_9 , x_10 , x_11 , x_12 , ) ? ;
            }
            Instruction::GetKernelNDrangeSubGroupCount(x_0, x_1, x_2, x_3, x_4, x_5, x_6) => {
                write ! ( f , "%{1:?} = Instruction::GetKernelNDrangeSubGroupCount {0:?} {2:?} {3:?} {4:?} {5:?} {6:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , x_6 , ) ? ;
            }
            Instruction::GetKernelNDrangeMaxSubGroupSize(x_0, x_1, x_2, x_3, x_4, x_5, x_6) => {
                write ! ( f , "%{1:?} = Instruction::GetKernelNDrangeMaxSubGroupSize {0:?} {2:?} {3:?} {4:?} {5:?} {6:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , x_6 , ) ? ;
            }
            Instruction::GetKernelWorkGroupSize(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GetKernelWorkGroupSize {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::GetKernelPreferredWorkGroupSizeMultiple(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write ! ( f , "%{1:?} = Instruction::GetKernelPreferredWorkGroupSizeMultiple {0:?} {2:?} {3:?} {4:?} {5:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , ) ? ;
            }
            Instruction::RetainEvent(x_0) => {
                write!(f, "Instruction::RetainEvent {0:?}", x_0,)?;
            }
            Instruction::ReleaseEvent(x_0) => {
                write!(f, "Instruction::ReleaseEvent {0:?}", x_0,)?;
            }
            Instruction::CreateUserEvent(x_0, x_1) => {
                write!(f, "%{1:?} = Instruction::CreateUserEvent {0:?}", x_0, x_1,)?;
            }
            Instruction::IsValidEvent(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::IsValidEvent {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SetUserEventStatus(x_0, x_1) => {
                write!(f, "Instruction::SetUserEventStatus {0:?} {1:?}", x_0, x_1,)?;
            }
            Instruction::CaptureEventProfilingInfo(x_0, x_1, x_2) => {
                write!(
                    f,
                    "Instruction::CaptureEventProfilingInfo {0:?} {1:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::GetDefaultQueue(x_0, x_1) => {
                write!(f, "%{1:?} = Instruction::GetDefaultQueue {0:?}", x_0, x_1,)?;
            }
            Instruction::BuildNDRange(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::BuildNDRange {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::ImageSparseSampleImplicitLod(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ImageSparseSampleImplicitLod {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::ImageSparseSampleExplicitLod(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ImageSparseSampleExplicitLod {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::ImageSparseSampleDrefImplicitLod(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write ! ( f , "%{1:?} = Instruction::ImageSparseSampleDrefImplicitLod {0:?} {2:?} {3:?} {4:?} {5:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , ) ? ;
            }
            Instruction::ImageSparseSampleDrefExplicitLod(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write ! ( f , "%{1:?} = Instruction::ImageSparseSampleDrefExplicitLod {0:?} {2:?} {3:?} {4:?} {5:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , ) ? ;
            }
            Instruction::ImageSparseSampleProjImplicitLod(x_0, x_1, x_2, x_3, x_4) => {
                write ! ( f , "%{1:?} = Instruction::ImageSparseSampleProjImplicitLod {0:?} {2:?} {3:?} {4:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , ) ? ;
            }
            Instruction::ImageSparseSampleProjExplicitLod(x_0, x_1, x_2, x_3, x_4) => {
                write ! ( f , "%{1:?} = Instruction::ImageSparseSampleProjExplicitLod {0:?} {2:?} {3:?} {4:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , ) ? ;
            }
            Instruction::ImageSparseSampleProjDrefImplicitLod(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write ! ( f , "%{1:?} = Instruction::ImageSparseSampleProjDrefImplicitLod {0:?} {2:?} {3:?} {4:?} {5:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , ) ? ;
            }
            Instruction::ImageSparseSampleProjDrefExplicitLod(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write ! ( f , "%{1:?} = Instruction::ImageSparseSampleProjDrefExplicitLod {0:?} {2:?} {3:?} {4:?} {5:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , ) ? ;
            }
            Instruction::ImageSparseFetch(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ImageSparseFetch {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::ImageSparseGather(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ImageSparseGather {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::ImageSparseDrefGather(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ImageSparseDrefGather {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::ImageSparseTexelsResident(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ImageSparseTexelsResident {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::NoLine => {
                write!(f, "Instruction::NoLine")?;
            }
            Instruction::AtomicFlagTestAndSet(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::AtomicFlagTestAndSet {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::AtomicFlagClear(x_0, x_1, x_2) => {
                write!(
                    f,
                    "Instruction::AtomicFlagClear {0:?} {1:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::ImageSparseRead(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ImageSparseRead {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::SizeOf(x_0, x_1, x_2) => {
                write!(f, "%{1:?} = Instruction::SizeOf {0:?} {2:?}", x_0, x_1, x_2,)?;
            }
            Instruction::TypePipeStorage(x_0) => {
                write!(f, "%{0:?} = Instruction::TypePipeStorage ", x_0,)?;
            }
            Instruction::ConstantPipeStorage(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ConstantPipeStorage {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::CreatePipeFromPipeStorage(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::CreatePipeFromPipeStorage {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::GetKernelLocalSizeForSubgroupCount(x_0, x_1, x_2, x_3, x_4, x_5, x_6) => {
                write ! ( f , "%{1:?} = Instruction::GetKernelLocalSizeForSubgroupCount {0:?} {2:?} {3:?} {4:?} {5:?} {6:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , x_6 , ) ? ;
            }
            Instruction::GetKernelMaxNumSubgroups(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GetKernelMaxNumSubgroups {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::TypeNamedBarrier(x_0) => {
                write!(f, "%{0:?} = Instruction::TypeNamedBarrier ", x_0,)?;
            }
            Instruction::NamedBarrierInitialize(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::NamedBarrierInitialize {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::MemoryNamedBarrier(x_0, x_1, x_2) => {
                write!(
                    f,
                    "Instruction::MemoryNamedBarrier {0:?} {1:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::ModuleProcessed(x_0) => {
                write!(f, "Instruction::ModuleProcessed {0:?}", x_0,)?;
            }
            Instruction::ExecutionModeId(x_0, x_1) => {
                write!(f, "Instruction::ExecutionModeId {0:?} {1:?}", x_0, x_1,)?;
            }
            Instruction::DecorateId(x_0, x_1) => {
                write!(f, "Instruction::DecorateId {0:?} {1:?}", x_0, x_1,)?;
            }
            Instruction::GroupNonUniformElect(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformElect {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::GroupNonUniformAll(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformAll {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::GroupNonUniformAny(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformAny {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::GroupNonUniformAllEqual(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformAllEqual {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::GroupNonUniformBroadcast(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformBroadcast {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::GroupNonUniformBroadcastFirst(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformBroadcastFirst {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::GroupNonUniformBallot(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformBallot {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::GroupNonUniformInverseBallot(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformInverseBallot {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::GroupNonUniformBallotBitExtract(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformBallotBitExtract {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::GroupNonUniformBallotBitCount(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformBallotBitCount {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::GroupNonUniformBallotFindLSB(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformBallotFindLSB {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::GroupNonUniformBallotFindMSB(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformBallotFindMSB {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::GroupNonUniformShuffle(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformShuffle {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::GroupNonUniformShuffleXor(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformShuffleXor {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::GroupNonUniformShuffleUp(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformShuffleUp {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::GroupNonUniformShuffleDown(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformShuffleDown {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::GroupNonUniformIAdd(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformIAdd {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::GroupNonUniformFAdd(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformFAdd {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::GroupNonUniformIMul(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformIMul {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::GroupNonUniformFMul(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformFMul {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::GroupNonUniformSMin(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformSMin {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::GroupNonUniformUMin(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformUMin {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::GroupNonUniformFMin(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformFMin {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::GroupNonUniformSMax(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformSMax {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::GroupNonUniformUMax(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformUMax {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::GroupNonUniformFMax(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformFMax {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::GroupNonUniformBitwiseAnd(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformBitwiseAnd {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::GroupNonUniformBitwiseOr(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformBitwiseOr {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::GroupNonUniformBitwiseXor(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformBitwiseXor {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::GroupNonUniformLogicalAnd(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformLogicalAnd {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::GroupNonUniformLogicalOr(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformLogicalOr {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::GroupNonUniformLogicalXor(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformLogicalXor {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::GroupNonUniformQuadBroadcast(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformQuadBroadcast {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::GroupNonUniformQuadSwap(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformQuadSwap {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::CopyLogical(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::CopyLogical {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::PtrEqual(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::PtrEqual {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::PtrNotEqual(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::PtrNotEqual {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::PtrDiff(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::PtrDiff {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::SubgroupBallotKHR(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupBallotKHR {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupFirstInvocationKHR(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupFirstInvocationKHR {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAllKHR(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAllKHR {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAnyKHR(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAnyKHR {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAllEqualKHR(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAllEqualKHR {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupReadInvocationKHR(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupReadInvocationKHR {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::GroupIAddNonUniformAMD(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupIAddNonUniformAMD {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::GroupFAddNonUniformAMD(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupFAddNonUniformAMD {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::GroupFMinNonUniformAMD(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupFMinNonUniformAMD {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::GroupUMinNonUniformAMD(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupUMinNonUniformAMD {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::GroupSMinNonUniformAMD(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupSMinNonUniformAMD {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::GroupFMaxNonUniformAMD(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupFMaxNonUniformAMD {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::GroupUMaxNonUniformAMD(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupUMaxNonUniformAMD {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::GroupSMaxNonUniformAMD(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupSMaxNonUniformAMD {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::FragmentMaskFetchAMD(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::FragmentMaskFetchAMD {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::FragmentFetchAMD(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::FragmentFetchAMD {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::ReadClockKHR(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ReadClockKHR {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::ImageSampleFootprintNV(x_0, x_1, x_2, x_3, x_4, x_5, x_6) => {
                write ! ( f , "%{1:?} = Instruction::ImageSampleFootprintNV {0:?} {2:?} {3:?} {4:?} {5:?} {6:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , x_6 , ) ? ;
            }
            Instruction::GroupNonUniformPartitionNV(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::GroupNonUniformPartitionNV {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::WritePackedPrimitiveIndices4x8NV(x_0, x_1) => {
                write!(
                    f,
                    "Instruction::WritePackedPrimitiveIndices4x8NV {0:?} {1:?}",
                    x_0, x_1,
                )?;
            }
            Instruction::ReportIntersectionNV(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ReportIntersectionNV {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::IgnoreIntersectionNV => {
                write!(f, "Instruction::IgnoreIntersectionNV")?;
            }
            Instruction::TerminateRayNV => {
                write!(f, "Instruction::TerminateRayNV")?;
            }
            Instruction::TraceNV(x_0, x_1, x_2, x_3, x_4, x_5, x_6, x_7, x_8, x_9, x_10) => {
                write ! ( f , "Instruction::TraceNV {0:?} {1:?} {2:?} {3:?} {4:?} {5:?} {6:?} {7:?} {8:?} {9:?} {10:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , x_6 , x_7 , x_8 , x_9 , x_10 , ) ? ;
            }
            Instruction::TypeAccelerationStructureNV(x_0) => {
                write!(f, "%{0:?} = Instruction::TypeAccelerationStructureNV ", x_0,)?;
            }
            Instruction::ExecuteCallableNV(x_0, x_1) => {
                write!(f, "Instruction::ExecuteCallableNV {0:?} {1:?}", x_0, x_1,)?;
            }
            Instruction::TypeCooperativeMatrixNV(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{0:?} = Instruction::TypeCooperativeMatrixNV {1:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::CooperativeMatrixLoadNV(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write!(
                    f,
                    "%{1:?} = Instruction::CooperativeMatrixLoadNV {0:?} {2:?} {3:?} {4:?} {5:?}",
                    x_0, x_1, x_2, x_3, x_4, x_5,
                )?;
            }
            Instruction::CooperativeMatrixStoreNV(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "Instruction::CooperativeMatrixStoreNV {0:?} {1:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::CooperativeMatrixMulAddNV(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::CooperativeMatrixMulAddNV {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::CooperativeMatrixLengthNV(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::CooperativeMatrixLengthNV {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::BeginInvocationInterlockEXT => {
                write!(f, "Instruction::BeginInvocationInterlockEXT")?;
            }
            Instruction::EndInvocationInterlockEXT => {
                write!(f, "Instruction::EndInvocationInterlockEXT")?;
            }
            Instruction::DemoteToHelperInvocationEXT => {
                write!(f, "Instruction::DemoteToHelperInvocationEXT")?;
            }
            Instruction::IsHelperInvocationEXT(x_0, x_1) => {
                write!(
                    f,
                    "%{1:?} = Instruction::IsHelperInvocationEXT {0:?}",
                    x_0, x_1,
                )?;
            }
            Instruction::SubgroupShuffleINTEL(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupShuffleINTEL {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::SubgroupShuffleDownINTEL(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupShuffleDownINTEL {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::SubgroupShuffleUpINTEL(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupShuffleUpINTEL {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::SubgroupShuffleXorINTEL(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupShuffleXorINTEL {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::SubgroupBlockReadINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupBlockReadINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupBlockWriteINTEL(x_0, x_1) => {
                write!(
                    f,
                    "Instruction::SubgroupBlockWriteINTEL {0:?} {1:?}",
                    x_0, x_1,
                )?;
            }
            Instruction::SubgroupImageBlockReadINTEL(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupImageBlockReadINTEL {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::SubgroupImageBlockWriteINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "Instruction::SubgroupImageBlockWriteINTEL {0:?} {1:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupImageMediaBlockReadINTEL(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupImageMediaBlockReadINTEL {0:?} {2:?} {3:?} {4:?} {5:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , ) ? ;
            }
            Instruction::SubgroupImageMediaBlockWriteINTEL(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "Instruction::SubgroupImageMediaBlockWriteINTEL {0:?} {1:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::UCountLeadingZerosINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::UCountLeadingZerosINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::UCountTrailingZerosINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::UCountTrailingZerosINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::AbsISubINTEL(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::AbsISubINTEL {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::AbsUSubINTEL(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::AbsUSubINTEL {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::IAddSatINTEL(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::IAddSatINTEL {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::UAddSatINTEL(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::UAddSatINTEL {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::IAverageINTEL(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::IAverageINTEL {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::UAverageINTEL(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::UAverageINTEL {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::IAverageRoundedINTEL(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::IAverageRoundedINTEL {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::UAverageRoundedINTEL(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::UAverageRoundedINTEL {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::ISubSatINTEL(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::ISubSatINTEL {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::USubSatINTEL(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::USubSatINTEL {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::IMul32x16INTEL(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::IMul32x16INTEL {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::UMul32x16INTEL(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::UMul32x16INTEL {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::DecorateString(x_0, x_1) => {
                write!(f, "Instruction::DecorateString {0:?} {1:?}", x_0, x_1,)?;
            }
            Instruction::DecorateStringGOOGLE(x_0, x_1) => {
                write!(f, "Instruction::DecorateStringGOOGLE {0:?} {1:?}", x_0, x_1,)?;
            }
            Instruction::MemberDecorateString(x_0, x_1, x_2) => {
                write!(
                    f,
                    "Instruction::MemberDecorateString {0:?} {1:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::MemberDecorateStringGOOGLE(x_0, x_1, x_2) => {
                write!(
                    f,
                    "Instruction::MemberDecorateStringGOOGLE {0:?} {1:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::VmeImageINTEL(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::VmeImageINTEL {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::TypeVmeImageINTEL(x_0, x_1) => {
                write!(f, "%{0:?} = Instruction::TypeVmeImageINTEL {1:?}", x_0, x_1,)?;
            }
            Instruction::TypeAvcImePayloadINTEL(x_0) => {
                write!(f, "%{0:?} = Instruction::TypeAvcImePayloadINTEL ", x_0,)?;
            }
            Instruction::TypeAvcRefPayloadINTEL(x_0) => {
                write!(f, "%{0:?} = Instruction::TypeAvcRefPayloadINTEL ", x_0,)?;
            }
            Instruction::TypeAvcSicPayloadINTEL(x_0) => {
                write!(f, "%{0:?} = Instruction::TypeAvcSicPayloadINTEL ", x_0,)?;
            }
            Instruction::TypeAvcMcePayloadINTEL(x_0) => {
                write!(f, "%{0:?} = Instruction::TypeAvcMcePayloadINTEL ", x_0,)?;
            }
            Instruction::TypeAvcMceResultINTEL(x_0) => {
                write!(f, "%{0:?} = Instruction::TypeAvcMceResultINTEL ", x_0,)?;
            }
            Instruction::TypeAvcImeResultINTEL(x_0) => {
                write!(f, "%{0:?} = Instruction::TypeAvcImeResultINTEL ", x_0,)?;
            }
            Instruction::TypeAvcImeResultSingleReferenceStreamoutINTEL(x_0) => {
                write!(
                    f,
                    "%{0:?} = Instruction::TypeAvcImeResultSingleReferenceStreamoutINTEL ",
                    x_0,
                )?;
            }
            Instruction::TypeAvcImeResultDualReferenceStreamoutINTEL(x_0) => {
                write!(
                    f,
                    "%{0:?} = Instruction::TypeAvcImeResultDualReferenceStreamoutINTEL ",
                    x_0,
                )?;
            }
            Instruction::TypeAvcImeSingleReferenceStreaminINTEL(x_0) => {
                write!(
                    f,
                    "%{0:?} = Instruction::TypeAvcImeSingleReferenceStreaminINTEL ",
                    x_0,
                )?;
            }
            Instruction::TypeAvcImeDualReferenceStreaminINTEL(x_0) => {
                write!(
                    f,
                    "%{0:?} = Instruction::TypeAvcImeDualReferenceStreaminINTEL ",
                    x_0,
                )?;
            }
            Instruction::TypeAvcRefResultINTEL(x_0) => {
                write!(f, "%{0:?} = Instruction::TypeAvcRefResultINTEL ", x_0,)?;
            }
            Instruction::TypeAvcSicResultINTEL(x_0) => {
                write!(f, "%{0:?} = Instruction::TypeAvcSicResultINTEL ", x_0,)?;
            }
            Instruction::SubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL {0:?} {2:?} {3:?}" , x_0 , x_1 , x_2 , x_3 , ) ? ;
            }
            Instruction::SubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL {0:?} {2:?} {3:?}" , x_0 , x_1 , x_2 , x_3 , ) ? ;
            }
            Instruction::SubgroupAvcMceGetDefaultInterShapePenaltyINTEL(x_0, x_1, x_2, x_3) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcMceGetDefaultInterShapePenaltyINTEL {0:?} {2:?} {3:?}" , x_0 , x_1 , x_2 , x_3 , ) ? ;
            }
            Instruction::SubgroupAvcMceSetInterShapePenaltyINTEL(x_0, x_1, x_2, x_3) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcMceSetInterShapePenaltyINTEL {0:?} {2:?} {3:?}" , x_0 , x_1 , x_2 , x_3 , ) ? ;
            }
            Instruction::SubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL(x_0, x_1, x_2, x_3) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL {0:?} {2:?} {3:?}" , x_0 , x_1 , x_2 , x_3 , ) ? ;
            }
            Instruction::SubgroupAvcMceSetInterDirectionPenaltyINTEL(x_0, x_1, x_2, x_3) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcMceSetInterDirectionPenaltyINTEL {0:?} {2:?} {3:?}" , x_0 , x_1 , x_2 , x_3 , ) ? ;
            }
            Instruction::SubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL(x_0, x_1, x_2, x_3) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL {0:?} {2:?} {3:?}" , x_0 , x_1 , x_2 , x_3 , ) ? ;
            }
            Instruction::SubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL {0:?} {2:?} {3:?}" , x_0 , x_1 , x_2 , x_3 , ) ? ;
            }
            Instruction::SubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL(x_0, x_1) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL {0:?}",
                    x_0, x_1,
                )?;
            }
            Instruction::SubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL(x_0, x_1) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL {0:?}" , x_0 , x_1 , ) ? ;
            }
            Instruction::SubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL(x_0, x_1) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL {0:?}",
                    x_0, x_1,
                )?;
            }
            Instruction::SubgroupAvcMceSetMotionVectorCostFunctionINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
                x_4,
                x_5,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcMceSetMotionVectorCostFunctionINTEL {0:?} {2:?} {3:?} {4:?} {5:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , ) ? ;
            }
            Instruction::SubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL(x_0, x_1, x_2, x_3) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL {0:?} {2:?} {3:?}" , x_0 , x_1 , x_2 , x_3 , ) ? ;
            }
            Instruction::SubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL(x_0, x_1) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL {0:?}" , x_0 , x_1 , ) ? ;
            }
            Instruction::SubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL(x_0, x_1) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL {0:?}" , x_0 , x_1 , ) ? ;
            }
            Instruction::SubgroupAvcMceSetAcOnlyHaarINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcMceSetAcOnlyHaarINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL {0:?} {2:?} {3:?}" , x_0 , x_1 , x_2 , x_3 , ) ? ;
            }
            Instruction::SubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL {0:?} {2:?} {3:?}" , x_0 , x_1 , x_2 , x_3 , ) ? ;
            }
            Instruction::SubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
                x_4,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL {0:?} {2:?} {3:?} {4:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , ) ? ;
            }
            Instruction::SubgroupAvcMceConvertToImePayloadINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcMceConvertToImePayloadINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAvcMceConvertToImeResultINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcMceConvertToImeResultINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAvcMceConvertToRefPayloadINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcMceConvertToRefPayloadINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAvcMceConvertToRefResultINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcMceConvertToRefResultINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAvcMceConvertToSicPayloadINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcMceConvertToSicPayloadINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAvcMceConvertToSicResultINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcMceConvertToSicResultINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAvcMceGetMotionVectorsINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcMceGetMotionVectorsINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAvcMceGetInterDistortionsINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcMceGetInterDistortionsINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAvcMceGetBestInterDistortionsINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcMceGetBestInterDistortionsINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAvcMceGetInterMajorShapeINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcMceGetInterMajorShapeINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAvcMceGetInterMinorShapeINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcMceGetInterMinorShapeINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAvcMceGetInterDirectionsINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcMceGetInterDirectionsINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAvcMceGetInterMotionVectorCountINTEL(x_0, x_1, x_2) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcMceGetInterMotionVectorCountINTEL {0:?} {2:?}" , x_0 , x_1 , x_2 , ) ? ;
            }
            Instruction::SubgroupAvcMceGetInterReferenceIdsINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcMceGetInterReferenceIdsINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
                x_4,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL {0:?} {2:?} {3:?} {4:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , ) ? ;
            }
            Instruction::SubgroupAvcImeInitializeINTEL(x_0, x_1, x_2, x_3, x_4) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcImeInitializeINTEL {0:?} {2:?} {3:?} {4:?}",
                    x_0, x_1, x_2, x_3, x_4,
                )?;
            }
            Instruction::SubgroupAvcImeSetSingleReferenceINTEL(x_0, x_1, x_2, x_3, x_4) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcImeSetSingleReferenceINTEL {0:?} {2:?} {3:?} {4:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , ) ? ;
            }
            Instruction::SubgroupAvcImeSetDualReferenceINTEL(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcImeSetDualReferenceINTEL {0:?} {2:?} {3:?} {4:?} {5:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , ) ? ;
            }
            Instruction::SubgroupAvcImeRefWindowSizeINTEL(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcImeRefWindowSizeINTEL {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::SubgroupAvcImeAdjustRefOffsetINTEL(x_0, x_1, x_2, x_3, x_4, x_5) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcImeAdjustRefOffsetINTEL {0:?} {2:?} {3:?} {4:?} {5:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , ) ? ;
            }
            Instruction::SubgroupAvcImeConvertToMcePayloadINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcImeConvertToMcePayloadINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAvcImeSetMaxMotionVectorCountINTEL(x_0, x_1, x_2, x_3) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcImeSetMaxMotionVectorCountINTEL {0:?} {2:?} {3:?}" , x_0 , x_1 , x_2 , x_3 , ) ? ;
            }
            Instruction::SubgroupAvcImeSetUnidirectionalMixDisableINTEL(x_0, x_1, x_2) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcImeSetUnidirectionalMixDisableINTEL {0:?} {2:?}" , x_0 , x_1 , x_2 , ) ? ;
            }
            Instruction::SubgroupAvcImeSetEarlySearchTerminationThresholdINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcImeSetEarlySearchTerminationThresholdINTEL {0:?} {2:?} {3:?}" , x_0 , x_1 , x_2 , x_3 , ) ? ;
            }
            Instruction::SubgroupAvcImeSetWeightedSadINTEL(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcImeSetWeightedSadINTEL {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::SubgroupAvcImeEvaluateWithSingleReferenceINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
                x_4,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcImeEvaluateWithSingleReferenceINTEL {0:?} {2:?} {3:?} {4:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , ) ? ;
            }
            Instruction::SubgroupAvcImeEvaluateWithDualReferenceINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
                x_4,
                x_5,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcImeEvaluateWithDualReferenceINTEL {0:?} {2:?} {3:?} {4:?} {5:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , ) ? ;
            }
            Instruction::SubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
                x_4,
                x_5,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL {0:?} {2:?} {3:?} {4:?} {5:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , ) ? ;
            }
            Instruction::SubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
                x_4,
                x_5,
                x_6,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL {0:?} {2:?} {3:?} {4:?} {5:?} {6:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , x_6 , ) ? ;
            }
            Instruction::SubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
                x_4,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL {0:?} {2:?} {3:?} {4:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , ) ? ;
            }
            Instruction::SubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
                x_4,
                x_5,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL {0:?} {2:?} {3:?} {4:?} {5:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , ) ? ;
            }
            Instruction::SubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
                x_4,
                x_5,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL {0:?} {2:?} {3:?} {4:?} {5:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , ) ? ;
            }
            Instruction::SubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
                x_4,
                x_5,
                x_6,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL {0:?} {2:?} {3:?} {4:?} {5:?} {6:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , x_6 , ) ? ;
            }
            Instruction::SubgroupAvcImeConvertToMceResultINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcImeConvertToMceResultINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAvcImeGetSingleReferenceStreaminINTEL(x_0, x_1, x_2) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcImeGetSingleReferenceStreaminINTEL {0:?} {2:?}" , x_0 , x_1 , x_2 , ) ? ;
            }
            Instruction::SubgroupAvcImeGetDualReferenceStreaminINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcImeGetDualReferenceStreaminINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAvcImeStripSingleReferenceStreamoutINTEL(x_0, x_1, x_2) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcImeStripSingleReferenceStreamoutINTEL {0:?} {2:?}" , x_0 , x_1 , x_2 , ) ? ;
            }
            Instruction::SubgroupAvcImeStripDualReferenceStreamoutINTEL(x_0, x_1, x_2) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcImeStripDualReferenceStreamoutINTEL {0:?} {2:?}" , x_0 , x_1 , x_2 , ) ? ;
            }
            Instruction::SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL {0:?} {2:?} {3:?}" , x_0 , x_1 , x_2 , x_3 , ) ? ;
            }
            Instruction::SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL {0:?} {2:?} {3:?}" , x_0 , x_1 , x_2 , x_3 , ) ? ;
            }
            Instruction::SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL {0:?} {2:?} {3:?}" , x_0 , x_1 , x_2 , x_3 , ) ? ;
            }
            Instruction::SubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
                x_4,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL {0:?} {2:?} {3:?} {4:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , ) ? ;
            }
            Instruction::SubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
                x_4,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL {0:?} {2:?} {3:?} {4:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , ) ? ;
            }
            Instruction::SubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
                x_4,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL {0:?} {2:?} {3:?} {4:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , ) ? ;
            }
            Instruction::SubgroupAvcImeGetBorderReachedINTEL(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcImeGetBorderReachedINTEL {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::SubgroupAvcImeGetTruncatedSearchIndicationINTEL(x_0, x_1, x_2) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcImeGetTruncatedSearchIndicationINTEL {0:?} {2:?}" , x_0 , x_1 , x_2 , ) ? ;
            }
            Instruction::SubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL(
                x_0,
                x_1,
                x_2,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL {0:?} {2:?}" , x_0 , x_1 , x_2 , ) ? ;
            }
            Instruction::SubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL(
                x_0,
                x_1,
                x_2,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL {0:?} {2:?}" , x_0 , x_1 , x_2 , ) ? ;
            }
            Instruction::SubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL(x_0, x_1, x_2) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL {0:?} {2:?}" , x_0 , x_1 , x_2 , ) ? ;
            }
            Instruction::SubgroupAvcFmeInitializeINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
                x_4,
                x_5,
                x_6,
                x_7,
                x_8,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcFmeInitializeINTEL {0:?} {2:?} {3:?} {4:?} {5:?} {6:?} {7:?} {8:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , x_6 , x_7 , x_8 , ) ? ;
            }
            Instruction::SubgroupAvcBmeInitializeINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
                x_4,
                x_5,
                x_6,
                x_7,
                x_8,
                x_9,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcBmeInitializeINTEL {0:?} {2:?} {3:?} {4:?} {5:?} {6:?} {7:?} {8:?} {9:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , x_6 , x_7 , x_8 , x_9 , ) ? ;
            }
            Instruction::SubgroupAvcRefConvertToMcePayloadINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcRefConvertToMcePayloadINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAvcRefSetBidirectionalMixDisableINTEL(x_0, x_1, x_2) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcRefSetBidirectionalMixDisableINTEL {0:?} {2:?}" , x_0 , x_1 , x_2 , ) ? ;
            }
            Instruction::SubgroupAvcRefSetBilinearFilterEnableINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcRefSetBilinearFilterEnableINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAvcRefEvaluateWithSingleReferenceINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
                x_4,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcRefEvaluateWithSingleReferenceINTEL {0:?} {2:?} {3:?} {4:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , ) ? ;
            }
            Instruction::SubgroupAvcRefEvaluateWithDualReferenceINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
                x_4,
                x_5,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcRefEvaluateWithDualReferenceINTEL {0:?} {2:?} {3:?} {4:?} {5:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , ) ? ;
            }
            Instruction::SubgroupAvcRefEvaluateWithMultiReferenceINTEL(x_0, x_1, x_2, x_3, x_4) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcRefEvaluateWithMultiReferenceINTEL {0:?} {2:?} {3:?} {4:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , ) ? ;
            }
            Instruction::SubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
                x_4,
                x_5,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL {0:?} {2:?} {3:?} {4:?} {5:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , ) ? ;
            }
            Instruction::SubgroupAvcRefConvertToMceResultINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcRefConvertToMceResultINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAvcSicInitializeINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcSicInitializeINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAvcSicConfigureSkcINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
                x_4,
                x_5,
                x_6,
                x_7,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcSicConfigureSkcINTEL {0:?} {2:?} {3:?} {4:?} {5:?} {6:?} {7:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , x_6 , x_7 , ) ? ;
            }
            Instruction::SubgroupAvcSicConfigureIpeLumaINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
                x_4,
                x_5,
                x_6,
                x_7,
                x_8,
                x_9,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcSicConfigureIpeLumaINTEL {0:?} {2:?} {3:?} {4:?} {5:?} {6:?} {7:?} {8:?} {9:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , x_6 , x_7 , x_8 , x_9 , ) ? ;
            }
            Instruction::SubgroupAvcSicConfigureIpeLumaChromaINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
                x_4,
                x_5,
                x_6,
                x_7,
                x_8,
                x_9,
                x_10,
                x_11,
                x_12,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcSicConfigureIpeLumaChromaINTEL {0:?} {2:?} {3:?} {4:?} {5:?} {6:?} {7:?} {8:?} {9:?} {10:?} {11:?} {12:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , x_6 , x_7 , x_8 , x_9 , x_10 , x_11 , x_12 , ) ? ;
            }
            Instruction::SubgroupAvcSicGetMotionVectorMaskINTEL(x_0, x_1, x_2, x_3) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcSicGetMotionVectorMaskINTEL {0:?} {2:?} {3:?}" , x_0 , x_1 , x_2 , x_3 , ) ? ;
            }
            Instruction::SubgroupAvcSicConvertToMcePayloadINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcSicConvertToMcePayloadINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAvcSicSetIntraLumaShapePenaltyINTEL(x_0, x_1, x_2, x_3) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcSicSetIntraLumaShapePenaltyINTEL {0:?} {2:?} {3:?}" , x_0 , x_1 , x_2 , x_3 , ) ? ;
            }
            Instruction::SubgroupAvcSicSetIntraLumaModeCostFunctionINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
                x_4,
                x_5,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcSicSetIntraLumaModeCostFunctionINTEL {0:?} {2:?} {3:?} {4:?} {5:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , ) ? ;
            }
            Instruction::SubgroupAvcSicSetIntraChromaModeCostFunctionINTEL(x_0, x_1, x_2, x_3) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcSicSetIntraChromaModeCostFunctionINTEL {0:?} {2:?} {3:?}" , x_0 , x_1 , x_2 , x_3 , ) ? ;
            }
            Instruction::SubgroupAvcSicSetBilinearFilterEnableINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcSicSetBilinearFilterEnableINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAvcSicSetSkcForwardTransformEnableINTEL(x_0, x_1, x_2, x_3) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcSicSetSkcForwardTransformEnableINTEL {0:?} {2:?} {3:?}" , x_0 , x_1 , x_2 , x_3 , ) ? ;
            }
            Instruction::SubgroupAvcSicSetBlockBasedRawSkipSadINTEL(x_0, x_1, x_2, x_3) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcSicSetBlockBasedRawSkipSadINTEL {0:?} {2:?} {3:?}" , x_0 , x_1 , x_2 , x_3 , ) ? ;
            }
            Instruction::SubgroupAvcSicEvaluateIpeINTEL(x_0, x_1, x_2, x_3) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcSicEvaluateIpeINTEL {0:?} {2:?} {3:?}",
                    x_0, x_1, x_2, x_3,
                )?;
            }
            Instruction::SubgroupAvcSicEvaluateWithSingleReferenceINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
                x_4,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcSicEvaluateWithSingleReferenceINTEL {0:?} {2:?} {3:?} {4:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , ) ? ;
            }
            Instruction::SubgroupAvcSicEvaluateWithDualReferenceINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
                x_4,
                x_5,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcSicEvaluateWithDualReferenceINTEL {0:?} {2:?} {3:?} {4:?} {5:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , ) ? ;
            }
            Instruction::SubgroupAvcSicEvaluateWithMultiReferenceINTEL(x_0, x_1, x_2, x_3, x_4) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcSicEvaluateWithMultiReferenceINTEL {0:?} {2:?} {3:?} {4:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , ) ? ;
            }
            Instruction::SubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL(
                x_0,
                x_1,
                x_2,
                x_3,
                x_4,
                x_5,
            ) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL {0:?} {2:?} {3:?} {4:?} {5:?}" , x_0 , x_1 , x_2 , x_3 , x_4 , x_5 , ) ? ;
            }
            Instruction::SubgroupAvcSicConvertToMceResultINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcSicConvertToMceResultINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAvcSicGetIpeLumaShapeINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcSicGetIpeLumaShapeINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAvcSicGetBestIpeLumaDistortionINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcSicGetBestIpeLumaDistortionINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAvcSicGetBestIpeChromaDistortionINTEL(x_0, x_1, x_2) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcSicGetBestIpeChromaDistortionINTEL {0:?} {2:?}" , x_0 , x_1 , x_2 , ) ? ;
            }
            Instruction::SubgroupAvcSicGetPackedIpeLumaModesINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcSicGetPackedIpeLumaModesINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAvcSicGetIpeChromaModeINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcSicGetIpeChromaModeINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::SubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL(x_0, x_1, x_2) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL {0:?} {2:?}" , x_0 , x_1 , x_2 , ) ? ;
            }
            Instruction::SubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL(x_0, x_1, x_2) => {
                write ! ( f , "%{1:?} = Instruction::SubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL {0:?} {2:?}" , x_0 , x_1 , x_2 , ) ? ;
            }
            Instruction::SubgroupAvcSicGetInterRawSadsINTEL(x_0, x_1, x_2) => {
                write!(
                    f,
                    "%{1:?} = Instruction::SubgroupAvcSicGetInterRawSadsINTEL {0:?} {2:?}",
                    x_0, x_1, x_2,
                )?;
            }
            Instruction::None(op_code) => {
                write!(f, "Instuction::None %{}", op_code)?;
            }
        }
        Ok(())
    }
}
