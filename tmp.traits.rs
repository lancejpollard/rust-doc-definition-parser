impl<T, const LANES: usize> AsMut<[T; LANES]> for Simd<T, LANES> where    T: SimdElement,    LaneCount<LANES>: SupportedLaneCount,
impl<T, const N: usize> AsMut<[T]> for [T; N]
impl<T, const LANES: usize> AsRef<[T; LANES]> for Simd<T, LANES> where    T: SimdElement,    LaneCount<LANES>: SupportedLaneCount,
impl<T, const N: usize> AsRef<[T]> for [T; N]
impl<T, const N: usize> Borrow<[T]> for [T; N]
impl<T, const N: usize> BorrowMut<[T]> for [T; N]
impl<T, const N: usize> Clone for [T; N] where    T: Clone,
impl<T, const N: usize> Debug for [T; N] where    T: Debug,
impl<T> Default for [T; 29] where    T: Default,
impl<T> Default for [T; 0]
impl<T> Default for [T; 5] where    T: Default,
impl<T> Default for [T; 15] where    T: Default,
impl<T> Default for [T; 25] where    T: Default,
impl<T> Default for [T; 31] where    T: Default,
impl<T> Default for [T; 6] where    T: Default,
impl<T> Default for [T; 12] where    T: Default,
impl<T> Default for [T; 17] where    T: Default,
impl<T> Default for [T; 3] where    T: Default,
impl<T> Default for [T; 30] where    T: Default,
impl<T> Default for [T; 32] where    T: Default,
impl<T> Default for [T; 2] where    T: Default,
impl<T> Default for [T; 13] where    T: Default,
impl<T> Default for [T; 23] where    T: Default,
impl<T> Default for [T; 14] where    T: Default,
impl<T> Default for [T; 21] where    T: Default,
impl<T> Default for [T; 16] where    T: Default,
impl<T> Default for [T; 11] where    T: Default,
impl<T> Default for [T; 22] where    T: Default,
impl<T> Default for [T; 26] where    T: Default,
impl<T> Default for [T; 4] where    T: Default,
impl<T> Default for [T; 1] where    T: Default,
impl<T> Default for [T; 19] where    T: Default,
impl<T> Default for [T; 9] where    T: Default,
impl<T> Default for [T; 28] where    T: Default,
impl<T> Default for [T; 18] where    T: Default,
impl<T> Default for [T; 10] where    T: Default,
impl<T> Default for [T; 20] where    T: Default,
impl<T> Default for [T; 7] where    T: Default,
impl<T> Default for [T; 24] where    T: Default,
impl<T> Default for [T; 8] where    T: Default,
impl<T> Default for [T; 27] where    T: Default,
impl<K, V, const N: usize> From<[(K, V); N]> for BTreeMap<K, V> where    K: Ord,
impl<K, V, const N: usize> From<[(K, V); N]> for HashMap<K, V, RandomState> where    K: Eq + Hash,
impl<T, const LANES: usize> From<[T; LANES]> for Simd<T, LANES> where    T: SimdElement,    LaneCount<LANES>: SupportedLaneCount,
impl<T, const N: usize> From<[T; N]> for LinkedList<T>
impl<T, const N: usize> From<[T; N]> for BTreeSet<T> where    T: Ord,
impl<T, const N: usize> From<[T; N]> for Vec<T, Global>
impl<T, const N: usize> From<[T; N]> for BinaryHeap<T> where    T: Ord,
impl<T, const N: usize> From<[T; N]> for Box<[T], Global>
impl<T, const N: usize> From<[T; N]> for VecDeque<T, Global>
impl<T, const N: usize> From<[T; N]> for HashSet<T, RandomState> where    T: Eq + Hash,
impl<T, const LANES: usize> From<[bool; LANES]> for Mask<T, LANES> where    T: MaskElement,    LaneCount<LANES>: SupportedLaneCount,
impl From<[u16; 8]> for Ipv6Addr
impl From<[u16; 8]> for IpAddr
impl From<[u8; 16]> for Ipv6Addr
impl From<[u8; 16]> for IpAddr
impl From<[u8; 4]> for Ipv4Addr
impl From<[u8; 4]> for IpAddr
impl<T, const LANES: usize> From<Mask<T, LANES>> for [bool; LANES] where    T: MaskElement,    LaneCount<LANES>: SupportedLaneCount,
impl<T, const LANES: usize> From<Simd<T, LANES>> for [T; LANES] where    T: SimdElement,    LaneCount<LANES>: SupportedLaneCount,
impl<T, const N: usize> Hash for [T; N] where    T: Hash,
impl<T, I, const N: usize> Index<I> for [T; N] where    [T]: Index<I>,
impl<T, I, const N: usize> IndexMut<I> for [T; N] where    [T]: IndexMut<I>,
impl<'a, T, const N: usize> IntoIterator for &'a mut [T; N]
impl<T, const N: usize> IntoIterator for [T; N]
impl<'a, T, const N: usize> IntoIterator for &'a [T; N]
impl<T, const N: usize> Ord for [T; N] where    T: Ord,
impl<'_, A, B, const N: usize> PartialEq<&'_ [B]> for [A; N] where    A: PartialEq<B>,
impl<'_, T, U, A, const N: usize> PartialEq<&'_ [U; N]> for Vec<T, A> where    A: Allocator,    T: PartialEq<U>,
impl<'_, T, U, A, const N: usize> PartialEq<&'_ [U; N]> for VecDeque<T, A> where    A: Allocator,    T: PartialEq<U>,
impl<'_, A, B, const N: usize> PartialEq<&'_ mut [B]> for [A; N] where    A: PartialEq<B>,
impl<'_, T, U, A, const N: usize> PartialEq<&'_ mut [U; N]> for VecDeque<T, A> where    A: Allocator,    T: PartialEq<U>,
impl<'_, A, B, const N: usize> PartialEq<[A; N]> for &'_ mut [B] where    B: PartialEq<A>,
impl<A, B, const N: usize> PartialEq<[A; N]> for [B] where    B: PartialEq<A>,
impl<'_, A, B, const N: usize> PartialEq<[A; N]> for &'_ [B] where    B: PartialEq<A>,
impl<A, B, const N: usize> PartialEq<[B; N]> for [A; N] where    A: PartialEq<B>,
impl<A, B, const N: usize> PartialEq<[B]> for [A; N] where    A: PartialEq<B>,
impl<T, U, A, const N: usize> PartialEq<[U; N]> for VecDeque<T, A> where    A: Allocator,    T: PartialEq<U>,
impl<T, U, A, const N: usize> PartialEq<[U; N]> for Vec<T, A> where    A: Allocator,    T: PartialEq<U>,
impl<T, const N: usize> PartialOrd<[T; N]> for [T; N] where    T: PartialOrd<T>,
impl<'a, 'b, const N: usize> Pattern<'a> for &'b [char; N]
impl<'a, const N: usize> Pattern<'a> for [char; N]
impl<T, const N: usize> SlicePattern for [T; N]
impl<'_, T, const N: usize> TryFrom<&'_ [T]> for [T; N] where    T: Copy,
impl<'_, T, const N: usize> TryFrom<&'_ mut [T]> for [T; N] where    T: Copy,
impl<'a, T, const N: usize> TryFrom<&'a [T]> for &'a [T; N]
impl<'a, T, const N: usize> TryFrom<&'a mut [T]> for &'a mut [T; N]
impl<T, A, const N: usize> TryFrom<Vec<T, A>> for [T; N] where    A: Allocator,
impl<'_> BitAnd<&'_ bool> for bool
impl<'_, '_> BitAnd<&'_ bool> for &'_ bool
impl<T, const LANES: usize> BitAnd<Mask<T, LANES>> for bool where    T: MaskElement,    LaneCount<LANES>: SupportedLaneCount,
impl BitAnd<bool> for bool
impl<'a> BitAnd<bool> for &'a bool
impl<T, const LANES: usize> BitAnd<bool> for Mask<T, LANES> where    T: MaskElement,    LaneCount<LANES>: SupportedLaneCount,
impl<'_> BitAndAssign<&'_ bool> for bool
impl BitAndAssign<bool> for bool
impl<T, const LANES: usize> BitAndAssign<bool> for Mask<T, LANES> where    T: MaskElement,    LaneCount<LANES>: SupportedLaneCount,
impl<'_, '_> BitOr<&'_ bool> for &'_ bool
impl<'_> BitOr<&'_ bool> for bool
impl<T, const LANES: usize> BitOr<Mask<T, LANES>> for bool where    T: MaskElement,    LaneCount<LANES>: SupportedLaneCount,
impl BitOr<bool> for bool
impl<T, const LANES: usize> BitOr<bool> for Mask<T, LANES> where    T: MaskElement,    LaneCount<LANES>: SupportedLaneCount,
impl<'a> BitOr<bool> for &'a bool
impl<'_> BitOrAssign<&'_ bool> for bool
impl<T, const LANES: usize> BitOrAssign<bool> for Mask<T, LANES> where    T: MaskElement,    LaneCount<LANES>: SupportedLaneCount,
impl BitOrAssign<bool> for bool
impl<'_> BitXor<&'_ bool> for bool
impl<'_, '_> BitXor<&'_ bool> for &'_ bool
impl<T, const LANES: usize> BitXor<Mask<T, LANES>> for bool where    T: MaskElement,    LaneCount<LANES>: SupportedLaneCount,
impl<T, const LANES: usize> BitXor<bool> for Mask<T, LANES> where    T: MaskElement,    LaneCount<LANES>: SupportedLaneCount,
impl BitXor<bool> for bool
impl<'a> BitXor<bool> for &'a bool
impl<'_> BitXorAssign<&'_ bool> for bool
impl BitXorAssign<bool> for bool
impl<T, const LANES: usize> BitXorAssign<bool> for Mask<T, LANES> where    T: MaskElement,    LaneCount<LANES>: SupportedLaneCount,
impl Clone for bool
impl Debug for bool
impl Default for bool
impl Display for bool
impl From<bool> for u64
impl From<bool> for i128
impl From<bool> for u32
impl From<bool> for i16
impl From<bool> for AtomicBool
impl From<bool> for i8
impl From<bool> for u128
impl From<bool> for usize
impl From<bool> for u8
impl From<bool> for isize
impl From<bool> for i32
impl From<bool> for u16
impl From<bool> for i64
impl FromStr for bool
impl Hash for bool
impl<'_> Not for &'_ bool
impl Not for bool
impl Ord for bool
impl PartialEq<bool> for bool
impl PartialOrd<bool> for bool
impl AsciiExt for char
impl Clone for char
impl Debug for char
impl Default for char
impl Display for char
impl<'a> Extend<&'a char> for String
impl Extend<char> for String
impl From<char> for u128
impl From<char> for u64
impl From<char> for u32
impl From<char> for String
impl From<u8> for char
impl<'a> FromIterator<&'a char> for String
impl<'a> FromIterator<char> for Cow<'a, str>
impl FromIterator<char> for String
impl FromStr for char
impl Hash for char
impl Ord for char
impl PartialEq<char> for char
impl PartialOrd<char> for char
impl<'a> Pattern<'a> for char
impl Step for char
impl ToString for char
impl TryFrom<char> for u8
impl TryFrom<u32> for char
impl<'_> Add<&'_ f32> for f32
impl<'_, '_> Add<&'_ f32> for &'_ f32
impl Add<f32> for f32
impl<'a> Add<f32> for &'a f32
impl<'_> AddAssign<&'_ f32> for f32
impl AddAssign<f32> for f32
impl Clone for f32
impl Debug for f32
impl Default for f32
impl Display for f32
impl<'_, '_> Div<&'_ f32> for &'_ f32
impl<'_> Div<&'_ f32> for f32
impl<'a> Div<f32> for &'a f32
impl Div<f32> for f32
impl<'_> DivAssign<&'_ f32> for f32
impl DivAssign<f32> for f32
impl From<f32> for f64
impl From<i16> for f32
impl From<i8> for f32
impl From<u16> for f32
impl From<u8> for f32
impl FromStr for f32
impl LowerExp for f32
impl<'_, '_> Mul<&'_ f32> for &'_ f32
impl<'_> Mul<&'_ f32> for f32
impl Mul<f32> for f32
impl<'a> Mul<f32> for &'a f32
impl<'_> MulAssign<&'_ f32> for f32
impl MulAssign<f32> for f32
impl<'_> Neg for &'_ f32
impl Neg for f32
impl PartialEq<f32> for f32
impl PartialOrd<f32> for f32
impl<'a> Product<&'a f32> for f32
impl Product<f32> for f32
impl<'_> Rem<&'_ f32> for f32
impl<'_, '_> Rem<&'_ f32> for &'_ f32
impl Rem<f32> for f32
impl<'a> Rem<f32> for &'a f32
impl<'_> RemAssign<&'_ f32> for f32
impl RemAssign<f32> for f32
impl SimdElement for f32
impl<'_, '_> Sub<&'_ f32> for &'_ f32
impl<'_> Sub<&'_ f32> for f32
impl Sub<f32> for f32
impl<'a> Sub<f32> for &'a f32
impl<'_> SubAssign<&'_ f32> for f32
impl SubAssign<f32> for f32
impl<'a> Sum<&'a f32> for f32
impl Sum<f32> for f32
impl UpperExp for f32
impl<'_> Add<&'_ f64> for f64
impl<'_, '_> Add<&'_ f64> for &'_ f64
impl<'a> Add<f64> for &'a f64
impl Add<f64> for f64
impl<'_> AddAssign<&'_ f64> for f64
impl AddAssign<f64> for f64
impl Clone for f64
impl Debug for f64
impl Default for f64
impl Display for f64
impl<'_> Div<&'_ f64> for f64
impl<'_, '_> Div<&'_ f64> for &'_ f64
impl Div<f64> for f64
impl<'a> Div<f64> for &'a f64
impl<'_> DivAssign<&'_ f64> for f64
impl DivAssign<f64> for f64
impl From<f32> for f64
impl From<i16> for f64
impl From<i32> for f64
impl From<i8> for f64
impl From<u16> for f64
impl From<u32> for f64
impl From<u8> for f64
impl FromStr for f64
impl LowerExp for f64
impl<'_> Mul<&'_ f64> for f64
impl<'_, '_> Mul<&'_ f64> for &'_ f64
impl<'a> Mul<f64> for &'a f64
impl Mul<f64> for f64
impl<'_> MulAssign<&'_ f64> for f64
impl MulAssign<f64> for f64
impl Neg for f64
impl<'_> Neg for &'_ f64
impl PartialEq<f64> for f64
impl PartialOrd<f64> for f64
impl<'a> Product<&'a f64> for f64
impl Product<f64> for f64
impl<'_, '_> Rem<&'_ f64> for &'_ f64
impl<'_> Rem<&'_ f64> for f64
impl<'a> Rem<f64> for &'a f64
impl Rem<f64> for f64
impl<'_> RemAssign<&'_ f64> for f64
impl RemAssign<f64> for f64
impl SimdElement for f64
impl<'_> Sub<&'_ f64> for f64
impl<'_, '_> Sub<&'_ f64> for &'_ f64
impl Sub<f64> for f64
impl<'a> Sub<f64> for &'a f64
impl<'_> SubAssign<&'_ f64> for f64
impl SubAssign<f64> for f64
impl<'a> Sum<&'a f64> for f64
impl Sum<f64> for f64
impl UpperExp for f64
impl<'_> Add<&'_ i32> for i32
impl<'_, '_> Add<&'_ i32> for &'_ i32
impl Add<i32> for i32
impl<'a> Add<i32> for &'a i32
impl<'_> AddAssign<&'_ i32> for i32
impl AddAssign<i32> for i32
impl Binary for i32
impl<'_> BitAnd<&'_ i32> for i32
impl<'_, '_> BitAnd<&'_ i32> for &'_ i32
impl<'a> BitAnd<i32> for &'a i32
impl BitAnd<i32> for i32
impl<'_> BitAndAssign<&'_ i32> for i32
impl BitAndAssign<i32> for i32
impl<'_> BitOr<&'_ i32> for i32
impl<'_, '_> BitOr<&'_ i32> for &'_ i32
impl BitOr<NonZeroI32> for i32
impl<'a> BitOr<i32> for &'a i32
impl BitOr<i32> for NonZeroI32
impl BitOr<i32> for i32
impl<'_> BitOrAssign<&'_ i32> for i32
impl BitOrAssign<i32> for NonZeroI32
impl BitOrAssign<i32> for i32
impl<'_, '_> BitXor<&'_ i32> for &'_ i32
impl<'_> BitXor<&'_ i32> for i32
impl BitXor<i32> for i32
impl<'a> BitXor<i32> for &'a i32
impl<'_> BitXorAssign<&'_ i32> for i32
impl BitXorAssign<i32> for i32
impl Clone for i32
impl Debug for i32
impl Default for i32
impl Display for i32
impl<'_, '_> Div<&'_ i32> for &'_ i32
impl<'_> Div<&'_ i32> for i32
impl Div<i32> for i32
impl<'a> Div<i32> for &'a i32
impl<'_> DivAssign<&'_ i32> for i32
impl DivAssign<i32> for i32
impl From<NonZeroI32> for i32
impl From<bool> for i32
impl From<i16> for i32
impl From<i32> for AtomicI32
impl From<i32> for f64
impl From<i32> for i64
impl From<i32> for i128
impl From<i8> for i32
impl From<u16> for i32
impl From<u8> for i32
impl FromStr for i32
impl Hash for i32
impl LowerExp for i32
impl LowerHex for i32
impl<'_, '_> Mul<&'_ i32> for &'_ i32
impl<'_> Mul<&'_ i32> for i32
impl<'a> Mul<i32> for &'a i32
impl Mul<i32> for i32
impl<'_> MulAssign<&'_ i32> for i32
impl MulAssign<i32> for i32
impl Neg for i32
impl<'_> Neg for &'_ i32
impl<'_> Not for &'_ i32
impl Not for i32
impl Octal for i32
impl Ord for i32
impl PartialEq<i32> for i32
impl PartialOrd<i32> for i32
impl<'a> Product<&'a i32> for i32
impl Product<i32> for i32
impl<'_, '_> Rem<&'_ i32> for &'_ i32
impl<'_> Rem<&'_ i32> for i32
impl<'a> Rem<i32> for &'a i32
impl Rem<i32> for i32
impl<'_> RemAssign<&'_ i32> for i32
impl RemAssign<i32> for i32
impl<'_, '_> Shl<&'_ i128> for &'_ i32
impl<'_> Shl<&'_ i128> for i32
impl<'_, '_> Shl<&'_ i16> for &'_ i32
impl<'_> Shl<&'_ i16> for i32
impl<'_, '_> Shl<&'_ i32> for &'_ i8
impl<'_, '_> Shl<&'_ i32> for &'_ usize
impl<'_> Shl<&'_ i32> for i32
impl<'_> Shl<&'_ i32> for isize
impl<'_, '_> Shl<&'_ i32> for &'_ u16
impl<'_, '_> Shl<&'_ i32> for &'_ u128
impl<'_, '_> Shl<&'_ i32> for &'_ u32
impl<'_, '_> Shl<&'_ i32> for &'_ i16
impl<'_> Shl<&'_ i32> for i8
impl<'_, '_> Shl<&'_ i32> for &'_ u64
impl<'_, '_> Shl<&'_ i32> for &'_ u8
impl<'_> Shl<&'_ i32> for u64
impl<'_> Shl<&'_ i32> for u16
impl<'_, '_> Shl<&'_ i32> for &'_ i64
impl<'_, '_> Shl<&'_ i32> for &'_ isize
impl<'_> Shl<&'_ i32> for i16
impl<'_> Shl<&'_ i32> for u8
impl<'_> Shl<&'_ i32> for u128
impl<'_> Shl<&'_ i32> for i64
impl<'_> Shl<&'_ i32> for usize
impl<'_, '_> Shl<&'_ i32> for &'_ i32
impl<'_, '_> Shl<&'_ i32> for &'_ i128
impl<'_> Shl<&'_ i32> for u32
impl<'_> Shl<&'_ i32> for i128
impl<'_, '_> Shl<&'_ i64> for &'_ i32
impl<'_> Shl<&'_ i64> for i32
impl<'_> Shl<&'_ i8> for i32
impl<'_, '_> Shl<&'_ i8> for &'_ i32
impl<'_, '_> Shl<&'_ isize> for &'_ i32
impl<'_> Shl<&'_ isize> for i32
impl<'_> Shl<&'_ u128> for i32
impl<'_, '_> Shl<&'_ u128> for &'_ i32
impl<'_> Shl<&'_ u16> for i32
impl<'_, '_> Shl<&'_ u16> for &'_ i32
impl<'_> Shl<&'_ u32> for i32
impl<'_, '_> Shl<&'_ u32> for &'_ i32
impl<'_> Shl<&'_ u64> for i32
impl<'_, '_> Shl<&'_ u64> for &'_ i32
impl<'_> Shl<&'_ u8> for i32
impl<'_, '_> Shl<&'_ u8> for &'_ i32
impl<'_, '_> Shl<&'_ usize> for &'_ i32
impl<'_> Shl<&'_ usize> for i32
impl<'a> Shl<i128> for &'a i32
impl Shl<i128> for i32
impl<'a> Shl<i16> for &'a i32
impl Shl<i16> for i32
impl<'a> Shl<i32> for &'a i8
impl Shl<i32> for i8
impl Shl<i32> for u16
impl<'a> Shl<i32> for &'a usize
impl Shl<i32> for isize
impl<'a> Shl<i32> for &'a i16
impl Shl<i32> for i128
impl<'a> Shl<i32> for &'a u8
impl Shl<i32> for u128
impl<'a> Shl<i32> for &'a isize
impl Shl<i32> for i32
impl<'a> Shl<i32> for &'a u64
impl Shl<i32> for u32
impl<'a> Shl<i32> for &'a u32
impl<'a> Shl<i32> for &'a i128
impl<'a> Shl<i32> for &'a i32
impl Shl<i32> for u64
impl Shl<i32> for u8
impl Shl<i32> for i64
impl<'a> Shl<i32> for &'a u16
impl<'a> Shl<i32> for &'a u128
impl Shl<i32> for i16
impl<'a> Shl<i32> for &'a i64
impl Shl<i32> for usize
impl<'a> Shl<i64> for &'a i32
impl Shl<i64> for i32
impl Shl<i8> for i32
impl<'a> Shl<i8> for &'a i32
impl Shl<isize> for i32
impl<'a> Shl<isize> for &'a i32
impl<'a> Shl<u128> for &'a i32
impl Shl<u128> for i32
impl<'a> Shl<u16> for &'a i32
impl Shl<u16> for i32
impl<'a> Shl<u32> for &'a i32
impl Shl<u32> for i32
impl<'a> Shl<u64> for &'a i32
impl Shl<u64> for i32
impl Shl<u8> for i32
impl<'a> Shl<u8> for &'a i32
impl Shl<usize> for i32
impl<'a> Shl<usize> for &'a i32
impl<'_> ShlAssign<&'_ i128> for i32
impl<'_> ShlAssign<&'_ i16> for i32
impl<'_> ShlAssign<&'_ i32> for i64
impl<'_> ShlAssign<&'_ i32> for u32
impl<'_> ShlAssign<&'_ i32> for u8
impl<'_> ShlAssign<&'_ i32> for i128
impl<'_> ShlAssign<&'_ i32> for u128
impl<'_> ShlAssign<&'_ i32> for usize
impl<'_> ShlAssign<&'_ i32> for u64
impl<'_> ShlAssign<&'_ i32> for u16
impl<'_> ShlAssign<&'_ i32> for isize
impl<'_> ShlAssign<&'_ i32> for i32
impl<'_> ShlAssign<&'_ i32> for i8
impl<'_> ShlAssign<&'_ i32> for i16
impl<'_> ShlAssign<&'_ i64> for i32
impl<'_> ShlAssign<&'_ i8> for i32
impl<'_> ShlAssign<&'_ isize> for i32
impl<'_> ShlAssign<&'_ u128> for i32
impl<'_> ShlAssign<&'_ u16> for i32
impl<'_> ShlAssign<&'_ u32> for i32
impl<'_> ShlAssign<&'_ u64> for i32
impl<'_> ShlAssign<&'_ u8> for i32
impl<'_> ShlAssign<&'_ usize> for i32
impl ShlAssign<i128> for i32
impl ShlAssign<i16> for i32
impl ShlAssign<i32> for i32
impl ShlAssign<i32> for u64
impl ShlAssign<i32> for i128
impl ShlAssign<i32> for i64
impl ShlAssign<i32> for u32
impl ShlAssign<i32> for u16
impl ShlAssign<i32> for isize
impl ShlAssign<i32> for i8
impl ShlAssign<i32> for u128
impl ShlAssign<i32> for u8
impl ShlAssign<i32> for usize
impl ShlAssign<i32> for i16
impl ShlAssign<i64> for i32
impl ShlAssign<i8> for i32
impl ShlAssign<isize> for i32
impl ShlAssign<u128> for i32
impl ShlAssign<u16> for i32
impl ShlAssign<u32> for i32
impl ShlAssign<u64> for i32
impl ShlAssign<u8> for i32
impl ShlAssign<usize> for i32
impl<'_, '_> Shr<&'_ i128> for &'_ i32
impl<'_> Shr<&'_ i128> for i32
impl<'_, '_> Shr<&'_ i16> for &'_ i32
impl<'_> Shr<&'_ i16> for i32
impl<'_> Shr<&'_ i32> for i64
impl<'_> Shr<&'_ i32> for u64
impl<'_, '_> Shr<&'_ i32> for &'_ usize
impl<'_> Shr<&'_ i32> for u32
impl<'_, '_> Shr<&'_ i32> for &'_ i128
impl<'_, '_> Shr<&'_ i32> for &'_ i16
impl<'_, '_> Shr<&'_ i32> for &'_ u32
impl<'_> Shr<&'_ i32> for i128
impl<'_> Shr<&'_ i32> for isize
impl<'_, '_> Shr<&'_ i32> for &'_ i8
impl<'_> Shr<&'_ i32> for u128
impl<'_, '_> Shr<&'_ i32> for &'_ u64
impl<'_, '_> Shr<&'_ i32> for &'_ u128
impl<'_> Shr<&'_ i32> for u16
impl<'_, '_> Shr<&'_ i32> for &'_ isize
impl<'_> Shr<&'_ i32> for i32
impl<'_> Shr<&'_ i32> for u8
impl<'_> Shr<&'_ i32> for usize
impl<'_, '_> Shr<&'_ i32> for &'_ u8
impl<'_, '_> Shr<&'_ i32> for &'_ i32
impl<'_> Shr<&'_ i32> for i16
impl<'_> Shr<&'_ i32> for i8
impl<'_, '_> Shr<&'_ i32> for &'_ u16
impl<'_, '_> Shr<&'_ i32> for &'_ i64
impl<'_> Shr<&'_ i64> for i32
impl<'_, '_> Shr<&'_ i64> for &'_ i32
impl<'_, '_> Shr<&'_ i8> for &'_ i32
impl<'_> Shr<&'_ i8> for i32
impl<'_, '_> Shr<&'_ isize> for &'_ i32
impl<'_> Shr<&'_ isize> for i32
impl<'_> Shr<&'_ u128> for i32
impl<'_, '_> Shr<&'_ u128> for &'_ i32
impl<'_, '_> Shr<&'_ u16> for &'_ i32
impl<'_> Shr<&'_ u16> for i32
impl<'_> Shr<&'_ u32> for i32
impl<'_, '_> Shr<&'_ u32> for &'_ i32
impl<'_, '_> Shr<&'_ u64> for &'_ i32
impl<'_> Shr<&'_ u64> for i32
impl<'_, '_> Shr<&'_ u8> for &'_ i32
impl<'_> Shr<&'_ u8> for i32
impl<'_> Shr<&'_ usize> for i32
impl<'_, '_> Shr<&'_ usize> for &'_ i32
impl<'a> Shr<i128> for &'a i32
impl Shr<i128> for i32
impl Shr<i16> for i32
impl<'a> Shr<i16> for &'a i32
impl<'a> Shr<i32> for &'a i128
impl<'a> Shr<i32> for &'a u8
impl Shr<i32> for u8
impl<'a> Shr<i32> for &'a u32
impl Shr<i32> for isize
impl<'a> Shr<i32> for &'a i16
impl Shr<i32> for i64
impl Shr<i32> for usize
impl Shr<i32> for u32
impl<'a> Shr<i32> for &'a isize
impl Shr<i32> for i16
impl Shr<i32> for u128
impl Shr<i32> for i32
impl Shr<i32> for i8
impl Shr<i32> for i128
impl Shr<i32> for u16
impl<'a> Shr<i32> for &'a u64
impl<'a> Shr<i32> for &'a u128
impl<'a> Shr<i32> for &'a i64
impl<'a> Shr<i32> for &'a usize
impl<'a> Shr<i32> for &'a i8
impl Shr<i32> for u64
impl<'a> Shr<i32> for &'a i32
impl<'a> Shr<i32> for &'a u16
impl Shr<i64> for i32
impl<'a> Shr<i64> for &'a i32
impl Shr<i8> for i32
impl<'a> Shr<i8> for &'a i32
impl<'a> Shr<isize> for &'a i32
impl Shr<isize> for i32
impl<'a> Shr<u128> for &'a i32
impl Shr<u128> for i32
impl<'a> Shr<u16> for &'a i32
impl Shr<u16> for i32
impl<'a> Shr<u32> for &'a i32
impl Shr<u32> for i32
impl<'a> Shr<u64> for &'a i32
impl Shr<u64> for i32
impl<'a> Shr<u8> for &'a i32
impl Shr<u8> for i32
impl Shr<usize> for i32
impl<'a> Shr<usize> for &'a i32
impl<'_> ShrAssign<&'_ i128> for i32
impl<'_> ShrAssign<&'_ i16> for i32
impl<'_> ShrAssign<&'_ i32> for u16
impl<'_> ShrAssign<&'_ i32> for i8
impl<'_> ShrAssign<&'_ i32> for i32
impl<'_> ShrAssign<&'_ i32> for usize
impl<'_> ShrAssign<&'_ i32> for i64
impl<'_> ShrAssign<&'_ i32> for isize
impl<'_> ShrAssign<&'_ i32> for i16
impl<'_> ShrAssign<&'_ i32> for u32
impl<'_> ShrAssign<&'_ i32> for u8
impl<'_> ShrAssign<&'_ i32> for u64
impl<'_> ShrAssign<&'_ i32> for i128
impl<'_> ShrAssign<&'_ i32> for u128
impl<'_> ShrAssign<&'_ i64> for i32
impl<'_> ShrAssign<&'_ i8> for i32
impl<'_> ShrAssign<&'_ isize> for i32
impl<'_> ShrAssign<&'_ u128> for i32
impl<'_> ShrAssign<&'_ u16> for i32
impl<'_> ShrAssign<&'_ u32> for i32
impl<'_> ShrAssign<&'_ u64> for i32
impl<'_> ShrAssign<&'_ u8> for i32
impl<'_> ShrAssign<&'_ usize> for i32
impl ShrAssign<i128> for i32
impl ShrAssign<i16> for i32
impl ShrAssign<i32> for u16
impl ShrAssign<i32> for i32
impl ShrAssign<i32> for i16
impl ShrAssign<i32> for u8
impl ShrAssign<i32> for i8
impl ShrAssign<i32> for u64
impl ShrAssign<i32> for u32
impl ShrAssign<i32> for u128
impl ShrAssign<i32> for usize
impl ShrAssign<i32> for i128
impl ShrAssign<i32> for i64
impl ShrAssign<i32> for isize
impl ShrAssign<i64> for i32
impl ShrAssign<i8> for i32
impl ShrAssign<isize> for i32
impl ShrAssign<u128> for i32
impl ShrAssign<u16> for i32
impl ShrAssign<u32> for i32
impl ShrAssign<u64> for i32
impl ShrAssign<u8> for i32
impl ShrAssign<usize> for i32
impl SimdElement for i32
impl Step for i32
impl<'_, '_> Sub<&'_ i32> for &'_ i32
impl<'_> Sub<&'_ i32> for i32
impl<'a> Sub<i32> for &'a i32
impl Sub<i32> for i32
impl<'_> SubAssign<&'_ i32> for i32
impl SubAssign<i32> for i32
impl<'a> Sum<&'a i32> for i32
impl Sum<i32> for i32
impl TryFrom<i128> for i32
impl TryFrom<i32> for u32
impl TryFrom<i32> for isize
impl TryFrom<i32> for NonZeroI32
impl TryFrom<i32> for usize
impl TryFrom<i32> for u64
impl TryFrom<i32> for u8
impl TryFrom<i32> for i8
impl TryFrom<i32> for u16
impl TryFrom<i32> for u128
impl TryFrom<i32> for i16
impl TryFrom<i64> for i32
impl TryFrom<isize> for i32
impl TryFrom<u128> for i32
impl TryFrom<u32> for i32
impl TryFrom<u64> for i32
impl TryFrom<usize> for i32
impl UpperExp for i32
impl UpperHex for i32
impl<'_> Add<&'_ isize> for isize
impl<'_, '_> Add<&'_ isize> for &'_ isize
impl Add<isize> for isize
impl<'a> Add<isize> for &'a isize
impl<'_> AddAssign<&'_ isize> for isize
impl AddAssign<isize> for isize
impl Binary for isize
impl<'_, '_> BitAnd<&'_ isize> for &'_ isize
impl<'_> BitAnd<&'_ isize> for isize
impl<'a> BitAnd<isize> for &'a isize
impl BitAnd<isize> for isize
impl<'_> BitAndAssign<&'_ isize> for isize
impl BitAndAssign<isize> for isize
impl<'_> BitOr<&'_ isize> for isize
impl<'_, '_> BitOr<&'_ isize> for &'_ isize
impl BitOr<NonZeroIsize> for isize
impl<'a> BitOr<isize> for &'a isize
impl BitOr<isize> for NonZeroIsize
impl BitOr<isize> for isize
impl<'_> BitOrAssign<&'_ isize> for isize
impl BitOrAssign<isize> for NonZeroIsize
impl BitOrAssign<isize> for isize
impl<'_, '_> BitXor<&'_ isize> for &'_ isize
impl<'_> BitXor<&'_ isize> for isize
impl BitXor<isize> for isize
impl<'a> BitXor<isize> for &'a isize
impl<'_> BitXorAssign<&'_ isize> for isize
impl BitXorAssign<isize> for isize
impl Clone for isize
impl Debug for isize
impl Default for isize
impl Display for isize
impl<'_, '_> Div<&'_ isize> for &'_ isize
impl<'_> Div<&'_ isize> for isize
impl Div<isize> for isize
impl<'a> Div<isize> for &'a isize
impl<'_> DivAssign<&'_ isize> for isize
impl DivAssign<isize> for isize
impl From<NonZeroIsize> for isize
impl From<bool> for isize
impl From<i16> for isize
impl From<i8> for isize
impl From<isize> for AtomicIsize
impl From<u8> for isize
impl FromStr for isize
impl Hash for isize
impl LowerExp for isize
impl LowerHex for isize
impl<'_, '_> Mul<&'_ isize> for &'_ isize
impl<'_> Mul<&'_ isize> for isize
impl Mul<isize> for isize
impl<'a> Mul<isize> for &'a isize
impl<'_> MulAssign<&'_ isize> for isize
impl MulAssign<isize> for isize
impl<'_> Neg for &'_ isize
impl Neg for isize
impl Not for isize
impl<'_> Not for &'_ isize
impl Octal for isize
impl Ord for isize
impl PartialEq<isize> for isize
impl PartialOrd<isize> for isize
impl<'a> Product<&'a isize> for isize
impl Product<isize> for isize
impl<'_, '_> Rem<&'_ isize> for &'_ isize
impl<'_> Rem<&'_ isize> for isize
impl<'a> Rem<isize> for &'a isize
impl Rem<isize> for isize
impl<'_> RemAssign<&'_ isize> for isize
impl RemAssign<isize> for isize
impl<'_> Shl<&'_ i128> for isize
impl<'_, '_> Shl<&'_ i128> for &'_ isize
impl<'_> Shl<&'_ i16> for isize
impl<'_, '_> Shl<&'_ i16> for &'_ isize
impl<'_> Shl<&'_ i32> for isize
impl<'_, '_> Shl<&'_ i32> for &'_ isize
impl<'_, '_> Shl<&'_ i64> for &'_ isize
impl<'_> Shl<&'_ i64> for isize
impl<'_, '_> Shl<&'_ i8> for &'_ isize
impl<'_> Shl<&'_ i8> for isize
impl<'_, '_> Shl<&'_ isize> for &'_ i16
impl<'_> Shl<&'_ isize> for i128
impl<'_> Shl<&'_ isize> for i64
impl<'_, '_> Shl<&'_ isize> for &'_ i64
impl<'_> Shl<&'_ isize> for usize
impl<'_, '_> Shl<&'_ isize> for &'_ i8
impl<'_> Shl<&'_ isize> for u8
impl<'_> Shl<&'_ isize> for u16
impl<'_, '_> Shl<&'_ isize> for &'_ i32
impl<'_, '_> Shl<&'_ isize> for &'_ isize
impl<'_> Shl<&'_ isize> for i32
impl<'_, '_> Shl<&'_ isize> for &'_ usize
impl<'_> Shl<&'_ isize> for u32
impl<'_, '_> Shl<&'_ isize> for &'_ u8
impl<'_> Shl<&'_ isize> for u128
impl<'_, '_> Shl<&'_ isize> for &'_ u128
impl<'_, '_> Shl<&'_ isize> for &'_ u64
impl<'_, '_> Shl<&'_ isize> for &'_ i128
impl<'_, '_> Shl<&'_ isize> for &'_ u16
impl<'_> Shl<&'_ isize> for i8
impl<'_> Shl<&'_ isize> for u64
impl<'_, '_> Shl<&'_ isize> for &'_ u32
impl<'_> Shl<&'_ isize> for i16
impl<'_> Shl<&'_ isize> for isize
impl<'_> Shl<&'_ u128> for isize
impl<'_, '_> Shl<&'_ u128> for &'_ isize
impl<'_> Shl<&'_ u16> for isize
impl<'_, '_> Shl<&'_ u16> for &'_ isize
impl<'_> Shl<&'_ u32> for isize
impl<'_, '_> Shl<&'_ u32> for &'_ isize
impl<'_, '_> Shl<&'_ u64> for &'_ isize
impl<'_> Shl<&'_ u64> for isize
impl<'_> Shl<&'_ u8> for isize
impl<'_, '_> Shl<&'_ u8> for &'_ isize
impl<'_> Shl<&'_ usize> for isize
impl<'_, '_> Shl<&'_ usize> for &'_ isize
impl Shl<i128> for isize
impl<'a> Shl<i128> for &'a isize
impl Shl<i16> for isize
impl<'a> Shl<i16> for &'a isize
impl Shl<i32> for isize
impl<'a> Shl<i32> for &'a isize
impl<'a> Shl<i64> for &'a isize
impl Shl<i64> for isize
impl Shl<i8> for isize
impl<'a> Shl<i8> for &'a isize
impl Shl<isize> for i128
impl Shl<isize> for usize
impl<'a> Shl<isize> for &'a u64
impl<'a> Shl<isize> for &'a u128
impl Shl<isize> for isize
impl Shl<isize> for i64
impl Shl<isize> for i16
impl<'a> Shl<isize> for &'a u32
impl Shl<isize> for i32
impl<'a> Shl<isize> for &'a i128
impl<'a> Shl<isize> for &'a u8
impl Shl<isize> for u128
impl<'a> Shl<isize> for &'a u16
impl Shl<isize> for i8
impl<'a> Shl<isize> for &'a usize
impl<'a> Shl<isize> for &'a i16
impl<'a> Shl<isize> for &'a i32
impl Shl<isize> for u8
impl Shl<isize> for u64
impl<'a> Shl<isize> for &'a isize
impl Shl<isize> for u32
impl Shl<isize> for u16
impl<'a> Shl<isize> for &'a i64
impl<'a> Shl<isize> for &'a i8
impl Shl<u128> for isize
impl<'a> Shl<u128> for &'a isize
impl<'a> Shl<u16> for &'a isize
impl Shl<u16> for isize
impl Shl<u32> for isize
impl<'a> Shl<u32> for &'a isize
impl Shl<u64> for isize
impl<'a> Shl<u64> for &'a isize
impl Shl<u8> for isize
impl<'a> Shl<u8> for &'a isize
impl<'a> Shl<usize> for &'a isize
impl Shl<usize> for isize
impl<'_> ShlAssign<&'_ i128> for isize
impl<'_> ShlAssign<&'_ i16> for isize
impl<'_> ShlAssign<&'_ i32> for isize
impl<'_> ShlAssign<&'_ i64> for isize
impl<'_> ShlAssign<&'_ i8> for isize
impl<'_> ShlAssign<&'_ isize> for i16
impl<'_> ShlAssign<&'_ isize> for i128
impl<'_> ShlAssign<&'_ isize> for u128
impl<'_> ShlAssign<&'_ isize> for u32
impl<'_> ShlAssign<&'_ isize> for u64
impl<'_> ShlAssign<&'_ isize> for i64
impl<'_> ShlAssign<&'_ isize> for u16
impl<'_> ShlAssign<&'_ isize> for usize
impl<'_> ShlAssign<&'_ isize> for i8
impl<'_> ShlAssign<&'_ isize> for isize
impl<'_> ShlAssign<&'_ isize> for i32
impl<'_> ShlAssign<&'_ isize> for u8
impl<'_> ShlAssign<&'_ u128> for isize
impl<'_> ShlAssign<&'_ u16> for isize
impl<'_> ShlAssign<&'_ u32> for isize
impl<'_> ShlAssign<&'_ u64> for isize
impl<'_> ShlAssign<&'_ u8> for isize
impl<'_> ShlAssign<&'_ usize> for isize
impl ShlAssign<i128> for isize
impl ShlAssign<i16> for isize
impl ShlAssign<i32> for isize
impl ShlAssign<i64> for isize
impl ShlAssign<i8> for isize
impl ShlAssign<isize> for u16
impl ShlAssign<isize> for u8
impl ShlAssign<isize> for i64
impl ShlAssign<isize> for u128
impl ShlAssign<isize> for u32
impl ShlAssign<isize> for u64
impl ShlAssign<isize> for i32
impl ShlAssign<isize> for usize
impl ShlAssign<isize> for i8
impl ShlAssign<isize> for i16
impl ShlAssign<isize> for isize
impl ShlAssign<isize> for i128
impl ShlAssign<u128> for isize
impl ShlAssign<u16> for isize
impl ShlAssign<u32> for isize
impl ShlAssign<u64> for isize
impl ShlAssign<u8> for isize
impl ShlAssign<usize> for isize
impl<'_, '_> Shr<&'_ i128> for &'_ isize
impl<'_> Shr<&'_ i128> for isize
impl<'_, '_> Shr<&'_ i16> for &'_ isize
impl<'_> Shr<&'_ i16> for isize
impl<'_> Shr<&'_ i32> for isize
impl<'_, '_> Shr<&'_ i32> for &'_ isize
impl<'_> Shr<&'_ i64> for isize
impl<'_, '_> Shr<&'_ i64> for &'_ isize
impl<'_, '_> Shr<&'_ i8> for &'_ isize
impl<'_> Shr<&'_ i8> for isize
impl<'_> Shr<&'_ isize> for isize
impl<'_> Shr<&'_ isize> for usize
impl<'_> Shr<&'_ isize> for i8
impl<'_, '_> Shr<&'_ isize> for &'_ i128
impl<'_, '_> Shr<&'_ isize> for &'_ u32
impl<'_> Shr<&'_ isize> for u8
impl<'_, '_> Shr<&'_ isize> for &'_ u64
impl<'_> Shr<&'_ isize> for i64
impl<'_, '_> Shr<&'_ isize> for &'_ usize
impl<'_, '_> Shr<&'_ isize> for &'_ i32
impl<'_, '_> Shr<&'_ isize> for &'_ u8
impl<'_, '_> Shr<&'_ isize> for &'_ isize
impl<'_, '_> Shr<&'_ isize> for &'_ i64
impl<'_> Shr<&'_ isize> for u128
impl<'_> Shr<&'_ isize> for u64
impl<'_> Shr<&'_ isize> for i32
impl<'_> Shr<&'_ isize> for u16
impl<'_, '_> Shr<&'_ isize> for &'_ i16
impl<'_, '_> Shr<&'_ isize> for &'_ u16
impl<'_> Shr<&'_ isize> for i16
impl<'_> Shr<&'_ isize> for u32
impl<'_> Shr<&'_ isize> for i128
impl<'_, '_> Shr<&'_ isize> for &'_ i8
impl<'_, '_> Shr<&'_ isize> for &'_ u128
impl<'_> Shr<&'_ u128> for isize
impl<'_, '_> Shr<&'_ u128> for &'_ isize
impl<'_> Shr<&'_ u16> for isize
impl<'_, '_> Shr<&'_ u16> for &'_ isize
impl<'_> Shr<&'_ u32> for isize
impl<'_, '_> Shr<&'_ u32> for &'_ isize
impl<'_> Shr<&'_ u64> for isize
impl<'_, '_> Shr<&'_ u64> for &'_ isize
impl<'_, '_> Shr<&'_ u8> for &'_ isize
impl<'_> Shr<&'_ u8> for isize
impl<'_, '_> Shr<&'_ usize> for &'_ isize
impl<'_> Shr<&'_ usize> for isize
impl Shr<i128> for isize
impl<'a> Shr<i128> for &'a isize
impl<'a> Shr<i16> for &'a isize
impl Shr<i16> for isize
impl Shr<i32> for isize
impl<'a> Shr<i32> for &'a isize
impl<'a> Shr<i64> for &'a isize
impl Shr<i64> for isize
impl<'a> Shr<i8> for &'a isize
impl Shr<i8> for isize
impl<'a> Shr<isize> for &'a usize
impl Shr<isize> for u32
impl<'a> Shr<isize> for &'a u64
impl<'a> Shr<isize> for &'a u8
impl<'a> Shr<isize> for &'a u128
impl<'a> Shr<isize> for &'a u16
impl<'a> Shr<isize> for &'a i8
impl Shr<isize> for u8
impl<'a> Shr<isize> for &'a i16
impl Shr<isize> for u64
impl Shr<isize> for i32
impl<'a> Shr<isize> for &'a i64
impl Shr<isize> for isize
impl Shr<isize> for u128
impl<'a> Shr<isize> for &'a isize
impl<'a> Shr<isize> for &'a i128
impl Shr<isize> for i64
impl<'a> Shr<isize> for &'a u32
impl Shr<isize> for i16
impl<'a> Shr<isize> for &'a i32
impl Shr<isize> for u16
impl Shr<isize> for i128
impl Shr<isize> for i8
impl Shr<isize> for usize
impl Shr<u128> for isize
impl<'a> Shr<u128> for &'a isize
impl<'a> Shr<u16> for &'a isize
impl Shr<u16> for isize
impl Shr<u32> for isize
impl<'a> Shr<u32> for &'a isize
impl<'a> Shr<u64> for &'a isize
impl Shr<u64> for isize
impl<'a> Shr<u8> for &'a isize
impl Shr<u8> for isize
impl Shr<usize> for isize
impl<'a> Shr<usize> for &'a isize
impl<'_> ShrAssign<&'_ i128> for isize
impl<'_> ShrAssign<&'_ i16> for isize
impl<'_> ShrAssign<&'_ i32> for isize
impl<'_> ShrAssign<&'_ i64> for isize
impl<'_> ShrAssign<&'_ i8> for isize
impl<'_> ShrAssign<&'_ isize> for i32
impl<'_> ShrAssign<&'_ isize> for u16
impl<'_> ShrAssign<&'_ isize> for i8
impl<'_> ShrAssign<&'_ isize> for u32
impl<'_> ShrAssign<&'_ isize> for isize
impl<'_> ShrAssign<&'_ isize> for u128
impl<'_> ShrAssign<&'_ isize> for usize
impl<'_> ShrAssign<&'_ isize> for i16
impl<'_> ShrAssign<&'_ isize> for i64
impl<'_> ShrAssign<&'_ isize> for i128
impl<'_> ShrAssign<&'_ isize> for u8
impl<'_> ShrAssign<&'_ isize> for u64
impl<'_> ShrAssign<&'_ u128> for isize
impl<'_> ShrAssign<&'_ u16> for isize
impl<'_> ShrAssign<&'_ u32> for isize
impl<'_> ShrAssign<&'_ u64> for isize
impl<'_> ShrAssign<&'_ u8> for isize
impl<'_> ShrAssign<&'_ usize> for isize
impl ShrAssign<i128> for isize
impl ShrAssign<i16> for isize
impl ShrAssign<i32> for isize
impl ShrAssign<i64> for isize
impl ShrAssign<i8> for isize
impl ShrAssign<isize> for i64
impl ShrAssign<isize> for u64
impl ShrAssign<isize> for u16
impl ShrAssign<isize> for u128
impl ShrAssign<isize> for u32
impl ShrAssign<isize> for usize
impl ShrAssign<isize> for i16
impl ShrAssign<isize> for u8
impl ShrAssign<isize> for isize
impl ShrAssign<isize> for i8
impl ShrAssign<isize> for i32
impl ShrAssign<isize> for i128
impl ShrAssign<u128> for isize
impl ShrAssign<u16> for isize
impl ShrAssign<u32> for isize
impl ShrAssign<u64> for isize
impl ShrAssign<u8> for isize
impl ShrAssign<usize> for isize
impl SimdElement for isize
impl Step for isize
impl<'_> Sub<&'_ isize> for isize
impl<'_, '_> Sub<&'_ isize> for &'_ isize
impl Sub<isize> for isize
impl<'a> Sub<isize> for &'a isize
impl<'_> SubAssign<&'_ isize> for isize
impl SubAssign<isize> for isize
impl<'a> Sum<&'a isize> for isize
impl Sum<isize> for isize
impl TryFrom<i128> for isize
impl TryFrom<i32> for isize
impl TryFrom<i64> for isize
impl TryFrom<isize> for i16
impl TryFrom<isize> for u64
impl TryFrom<isize> for i128
impl TryFrom<isize> for NonZeroIsize
impl TryFrom<isize> for u16
impl TryFrom<isize> for u128
impl TryFrom<isize> for i32
impl TryFrom<isize> for i64
impl TryFrom<isize> for i8
impl TryFrom<isize> for usize
impl TryFrom<isize> for u32
impl TryFrom<isize> for u8
impl TryFrom<u128> for isize
impl TryFrom<u16> for isize
impl TryFrom<u32> for isize
impl TryFrom<u64> for isize
impl TryFrom<usize> for isize
impl UpperExp for isize
impl UpperHex for isize
impl AsRef<OsStr> for OsString
impl AsRef<Path> for OsString
impl Borrow<OsStr> for OsString
impl Clone for OsString
impl Debug for OsString
impl Default for OsString
impl Deref for OsString
impl DerefMut for OsString
impl<'a> Extend<&'a OsStr> for OsString
impl<'a> Extend<Cow<'a, OsStr>> for OsString
impl Extend<OsString> for OsString
impl<T: ?Sized + AsRef<OsStr>> From<&'_ T> for OsString
impl<'a> From<&'a OsString> for Cow<'a, OsStr>
impl From<Box<OsStr, Global>> for OsString
impl<'a> From<Cow<'a, OsStr>> for OsString
impl From<OsString> for Box<OsStr>
impl From<OsString> for Arc<OsStr>
impl From<OsString> for Rc<OsStr>
impl<'a> From<OsString> for Cow<'a, OsStr>
impl From<OsString> for PathBuf
impl From<PathBuf> for OsString
impl From<String> for OsString
impl<'a> FromIterator<&'a OsStr> for OsString
impl<'a> FromIterator<Cow<'a, OsStr>> for OsString
impl FromIterator<OsString> for OsString
impl FromStr for OsString
impl Hash for OsString
impl Index<RangeFull> for OsString
impl IndexMut<RangeFull> for OsString
impl Ord for OsString
impl OsStringExt for OsString
impl OsStringExt for OsString
impl OsStringExt for OsString
impl PartialEq<&'_ str> for OsString
impl<'a, 'b> PartialEq<&'a OsStr> for OsString
impl<'a, 'b> PartialEq<&'a Path> for OsString
impl<'a, 'b> PartialEq<Cow<'a, OsStr>> for OsString
impl<'a, 'b> PartialEq<Cow<'a, Path>> for OsString
impl<'a, 'b> PartialEq<OsStr> for OsString
impl PartialEq<OsString> for OsString
impl PartialEq<OsString> for str
impl<'a> PartialEq<OsString> for &'a str
impl<'a, 'b> PartialEq<OsString> for OsStr
impl<'a, 'b> PartialEq<OsString> for &'a OsStr
impl<'a, 'b> PartialEq<OsString> for Cow<'a, OsStr>
impl<'a, 'b> PartialEq<OsString> for PathBuf
impl<'a, 'b> PartialEq<OsString> for Path
impl<'a, 'b> PartialEq<OsString> for &'a Path
impl<'a, 'b> PartialEq<OsString> for Cow<'a, Path>
impl<'a, 'b> PartialEq<Path> for OsString
impl<'a, 'b> PartialEq<PathBuf> for OsString
impl PartialEq<str> for OsString
impl<'a, 'b> PartialOrd<&'a OsStr> for OsString
impl<'a, 'b> PartialOrd<&'a Path> for OsString
impl<'a, 'b> PartialOrd<Cow<'a, OsStr>> for OsString
impl<'a, 'b> PartialOrd<Cow<'a, Path>> for OsString
impl<'a, 'b> PartialOrd<OsStr> for OsString
impl PartialOrd<OsString> for OsString
impl<'a, 'b> PartialOrd<OsString> for OsStr
impl<'a, 'b> PartialOrd<OsString> for &'a OsStr
impl<'a, 'b> PartialOrd<OsString> for Cow<'a, OsStr>
impl<'a, 'b> PartialOrd<OsString> for PathBuf
impl<'a, 'b> PartialOrd<OsString> for Path
impl<'a, 'b> PartialOrd<OsString> for &'a Path
impl<'a, 'b> PartialOrd<OsString> for Cow<'a, Path>
impl<'a, 'b> PartialOrd<Path> for OsString
impl<'a, 'b> PartialOrd<PathBuf> for OsString
impl PartialOrd<str> for OsString
impl Clone for Literal
impl Debug for Literal
impl Display for Literal
impl From<Literal> for TokenTree
impl FromStr for Literal
impl ToString for Literal
impl Clone for TokenStream
impl Debug for TokenStream
impl Default for TokenStream
impl Display for TokenStream
impl Extend<TokenStream> for TokenStream
impl Extend<TokenTree> for TokenStream
impl From<TokenTree> for TokenStream
impl FromIterator<TokenStream> for TokenStream
impl FromIterator<TokenTree> for TokenStream
impl FromStr for TokenStream
impl IntoIterator for TokenStream
impl ToString for TokenStream
impl<T, const N: usize> AsMut<[T]> for [T; N]
impl<T, const LANES: usize> AsMut<[T]> for Simd<T, LANES> where    T: SimdElement,    LaneCount<LANES>: SupportedLaneCount,
impl<T> AsMut<[T]> for [T]
impl<T, A> AsMut<[T]> for Vec<T, A> where    A: Allocator,
impl<T, const N: usize> AsRef<[T]> for [T; N]
impl<'_, T> AsRef<[T]> for IterMut<'_, T>
impl<'_, T> AsRef<[T]> for Iter<'_, T>
impl<T, const LANES: usize> AsRef<[T]> for Simd<T, LANES> where    T: SimdElement,    LaneCount<LANES>: SupportedLaneCount,
impl<T> AsRef<[T]> for [T]
impl<T, A> AsRef<[T]> for Vec<T, A> where    A: Allocator,
impl<T, A> AsRef<[T]> for IntoIter<T, A> where    A: Allocator,
impl<'a, T, A> AsRef<[T]> for Drain<'a, T, A> where    A: Allocator,
impl AsRef<[u8]> for str
impl AsRef<[u8]> for String
impl<'a> AsRef<[u8]> for Drain<'a>
impl AsciiExt for [u8]
impl<T, const N: usize> Borrow<[T]> for [T; N]
impl<T> Borrow<[T]> for Vec<T, Global>
impl<T, const N: usize> BorrowMut<[T]> for [T; N]
impl<T> BorrowMut<[T]> for Vec<T, Global>
impl BufRead for &[u8]
impl<T, V> Concat<T> for [V] where    T: Clone,    V: Borrow<[T]>,
impl<S> Concat<str> for [S] where    S: Borrow<str>,
impl<T> Debug for [T] where    T: Debug,
impl<'_, T> Default for &'_ [T]
impl<'_, T> Default for &'_ mut [T]
impl<'_, T> From<&'_ [T]> for Rc<[T]> where    T: Clone,
impl<'_, T> From<&'_ [T]> for Box<[T], Global> where    T: Copy,
impl<'_, T> From<&'_ [T]> for Vec<T, Global> where    T: Clone,
impl<'_, T> From<&'_ [T]> for Arc<[T]> where    T: Clone,
impl<'_, T> From<&'_ mut [T]> for Vec<T, Global> where    T: Clone,
impl<'a, T> From<&'a [T]> for Cow<'a, [T]> where    T: Clone,
impl<T> Hash for [T] where    T: Hash,
impl<T, I> Index<I> for [T] where    I: SliceIndex<[T]>,
impl<T, I> IndexMut<I> for [T] where    I: SliceIndex<[T]>,
impl<'a, T> IntoIterator for &'a [T]
impl<'a, T> IntoIterator for &'a mut [T]
impl<'_, T, V> Join<&'_ [T]> for [V] where    T: Clone,    V: Borrow<[T]>,
impl<'_, T, V> Join<&'_ T> for [V] where    T: Clone,    V: Borrow<[T]>,
impl<'_, S> Join<&'_ str> for [S] where    S: Borrow<str>,
impl<T> Ord for [T] where    T: Ord,
impl<'_, A, B, const N: usize> PartialEq<&'_ [B]> for [A; N] where    A: PartialEq<B>,
impl<'_, '_, T, U> PartialEq<&'_ [U]> for Cow<'_, [T]> where    T: PartialEq<U> + Clone,
impl<'_, T, U, A> PartialEq<&'_ [U]> for VecDeque<T, A> where    A: Allocator,    T: PartialEq<U>,
impl<'_, T, U, A> PartialEq<&'_ [U]> for Vec<T, A> where    A: Allocator,    T: PartialEq<U>,
impl<'_, A, B, const N: usize> PartialEq<&'_ mut [B]> for [A; N] where    A: PartialEq<B>,
impl<'_, T, U, A> PartialEq<&'_ mut [U]> for Vec<T, A> where    A: Allocator,    T: PartialEq<U>,
impl<'_, T, U, A> PartialEq<&'_ mut [U]> for VecDeque<T, A> where    A: Allocator,    T: PartialEq<U>,
impl<'_, '_, T, U> PartialEq<&'_ mut [U]> for Cow<'_, [T]> where    T: PartialEq<U> + Clone,
impl<'_, A, B, const N: usize> PartialEq<[A; N]> for &'_ mut [B] where    B: PartialEq<A>,
impl<A, B, const N: usize> PartialEq<[A; N]> for [B] where    B: PartialEq<A>,
impl<'_, A, B, const N: usize> PartialEq<[A; N]> for &'_ [B] where    B: PartialEq<A>,
impl<A, B, const N: usize> PartialEq<[B]> for [A; N] where    A: PartialEq<B>,
impl<A, B> PartialEq<[B]> for [A] where    A: PartialEq<B>,
impl<T, U, A> PartialEq<[U]> for Vec<T, A> where    A: Allocator,    T: PartialEq<U>,
impl<'_, T, U, A> PartialEq<Vec<U, A>> for &'_ [T] where    A: Allocator,    T: PartialEq<U>,
impl<T, U, A> PartialEq<Vec<U, A>> for [T] where    A: Allocator,    T: PartialEq<U>,
impl<'_, T, U, A> PartialEq<Vec<U, A>> for &'_ mut [T] where    A: Allocator,    T: PartialEq<U>,
impl<T> PartialOrd<[T]> for [T] where    T: PartialOrd<T>,
impl<'a, 'b> Pattern<'a> for &'b [char]
impl Read for &[u8]
impl<T> SliceIndex<[T]> for RangeFrom<usize>
impl<T> SliceIndex<[T]> for RangeInclusive<usize>
impl<T> SliceIndex<[T]> for RangeToInclusive<usize>
impl<T> SliceIndex<[T]> for usize
impl<T> SliceIndex<[T]> for RangeFull
impl<T> SliceIndex<[T]> for Range<usize>
impl<T> SliceIndex<[T]> for RangeTo<usize>
impl<T> SliceIndex<[T]> for (Bound<usize>, Bound<usize>)
impl<T> SlicePattern for [T]
impl<T> ToOwned for [T] where    T: Clone,
impl<'a> ToSocketAddrs for &'a [SocketAddr]
impl<'_, T, const N: usize> TryFrom<&'_ [T]> for [T; N] where    T: Copy,
impl<'_, T, const N: usize> TryFrom<&'_ mut [T]> for [T; N] where    T: Copy,
impl<'a, T, const N: usize> TryFrom<&'a [T]> for &'a [T; N]
impl<'a, T, const N: usize> TryFrom<&'a mut [T]> for &'a mut [T; N]
impl Write for &mut [u8]
impl Clone for Layout
impl Debug for Layout
impl PartialEq<Layout> for Layout
impl Allocator for System
impl Clone for System
impl Debug for System
impl Default for System
impl GlobalAlloc for System
impl Debug for dyn Any + 'static
impl Debug for dyn Any + Send + Sync + 'static
impl Debug for dyn Any + Send + 'static
impl Clone for TypeId
impl Debug for TypeId
impl Hash for TypeId
impl Ord for TypeId
impl PartialEq<TypeId> for TypeId
impl PartialOrd<TypeId> for TypeId
impl<T, const N: usize> Clone for IntoIter<T, N> where    T: Clone,
impl<T, const N: usize> Debug for IntoIter<T, N> where    T: Debug,
impl<T, const N: usize> DoubleEndedIterator for IntoIter<T, N>
impl<T, const N: usize> Drop for IntoIter<T, N>
impl<T, const N: usize> ExactSizeIterator for IntoIter<T, N>
impl<T, const N: usize> Iterator for IntoIter<T, N>
impl Clone for TryFromSliceError
impl Debug for TryFromSliceError
impl Display for TryFromSliceError
impl Error for TryFromSliceError
impl From<Infallible> for TryFromSliceError
impl Clone for EscapeDefault
impl Debug for EscapeDefault
impl Display for EscapeDefault
impl DoubleEndedIterator for EscapeDefault
impl ExactSizeIterator for EscapeDefault
impl Iterator for EscapeDefault
impl<'a> Add<&'a str> for Cow<'a, str>
impl<'a> Add<Cow<'a, str>> for Cow<'a, str>
impl<'a> AddAssign<&'a str> for Cow<'a, str>
impl<'a> AddAssign<Cow<'a, str>> for Cow<'a, str>
impl AsRef<Path> for Cow<'_, OsStr>
impl<'_, T> AsRef<T> for Cow<'_, T> where    T: ToOwned + ?Sized,
impl<'a, B> Borrow<B> for Cow<'a, B> where    B: ToOwned + ?Sized,    <B as ToOwned>::Owned: 'a,
impl<'_, B> Clone for Cow<'_, B> where    B: ToOwned + ?Sized,
impl<'_, B> Debug for Cow<'_, B> where    B: Debug + ToOwned + ?Sized,    <B as ToOwned>::Owned: Debug,
impl<'_, B> Default for Cow<'_, B> where    B: ToOwned + ?Sized,    <B as ToOwned>::Owned: Default,
impl<'_, B> Deref for Cow<'_, B> where    B: ToOwned + ?Sized,    <B as ToOwned>::Owned: Borrow<B>,
impl<'_, B> Display for Cow<'_, B> where    B: Display + ToOwned + ?Sized,    <B as ToOwned>::Owned: Display,
impl<'a> Extend<Cow<'a, OsStr>> for OsString
impl<'a> Extend<Cow<'a, str>> for String
impl<'a, T> From<&'a [T]> for Cow<'a, [T]> where    T: Clone,
impl<'a> From<&'a CStr> for Cow<'a, CStr>
impl<'a> From<&'a CString> for Cow<'a, CStr>
impl<'a> From<&'a OsStr> for Cow<'a, OsStr>
impl<'a> From<&'a OsString> for Cow<'a, OsStr>
impl<'a> From<&'a Path> for Cow<'a, Path>
impl<'a> From<&'a PathBuf> for Cow<'a, Path>
impl<'a> From<&'a String> for Cow<'a, str>
impl<'a, T> From<&'a Vec<T, Global>> for Cow<'a, [T]> where    T: Clone,
impl<'a> From<&'a str> for Cow<'a, str>
impl<'a> From<CString> for Cow<'a, CStr>
impl<'_, T> From<Cow<'_, [T]>> for Box<[T], Global> where    T: Copy,
impl From<Cow<'_, CStr>> for Box<CStr>
impl From<Cow<'_, OsStr>> for Box<OsStr>
impl From<Cow<'_, Path>> for Box<Path>
impl<'_> From<Cow<'_, str>> for Box<str, Global>
impl<'a, T> From<Cow<'a, [T]>> for Vec<T, Global> where    [T]: ToOwned,    <[T] as ToOwned>::Owned == Vec<T, Global>,
impl<'a, B> From<Cow<'a, B>> for Arc<B> where    B: ToOwned + ?Sized,    Arc<B>: From<&'a B>,    Arc<B>: From<<B as ToOwned>::Owned>,
impl<'a, B> From<Cow<'a, B>> for Rc<B> where    B: ToOwned + ?Sized,    Rc<B>: From<&'a B>,    Rc<B>: From<<B as ToOwned>::Owned>,
impl<'a> From<Cow<'a, CStr>> for CString
impl<'a> From<Cow<'a, OsStr>> for OsString
impl<'a> From<Cow<'a, Path>> for PathBuf
impl<'a> From<Cow<'a, str>> for String
impl<'a> From<Cow<'a, str>> for Box<dyn Error>
impl<'a, 'b> From<Cow<'b, str>> for Box<dyn Error + Send + Sync + 'a>
impl<'a> From<OsString> for Cow<'a, OsStr>
impl<'a> From<PathBuf> for Cow<'a, Path>
impl<'a> From<String> for Cow<'a, str>
impl<'a, T> From<Vec<T, Global>> for Cow<'a, [T]> where    T: Clone,
impl<'a, 'b> FromIterator<&'b str> for Cow<'a, str>
impl<'a> FromIterator<Cow<'a, OsStr>> for OsString
impl<'a> FromIterator<Cow<'a, str>> for String
impl<'a> FromIterator<String> for Cow<'a, str>
impl<'a, T> FromIterator<T> for Cow<'a, [T]> where    T: Clone,
impl<'a> FromIterator<char> for Cow<'a, str>
impl<'_, B> Hash for Cow<'_, B> where    B: Hash + ToOwned + ?Sized,
impl<'_, B> Ord for Cow<'_, B> where    B: Ord + ToOwned + ?Sized,
impl<'_, '_, T, U> PartialEq<&'_ [U]> for Cow<'_, [T]> where    T: PartialEq<U> + Clone,
impl<'_, '_, T, U> PartialEq<&'_ mut [U]> for Cow<'_, [T]> where    T: PartialEq<U> + Clone,
impl<'a, 'b> PartialEq<&'a Path> for Cow<'b, OsStr>
impl<'a, 'b> PartialEq<&'b OsStr> for Cow<'a, OsStr>
impl<'a, 'b> PartialEq<&'b OsStr> for Cow<'a, Path>
impl<'a, 'b> PartialEq<&'b Path> for Cow<'a, Path>
impl<'a, 'b> PartialEq<&'b str> for Cow<'a, str>
impl<'a, 'b> PartialEq<Cow<'a, OsStr>> for OsStr
impl<'a, 'b> PartialEq<Cow<'a, OsStr>> for &'b OsStr
impl<'a, 'b> PartialEq<Cow<'a, OsStr>> for OsString
impl<'a, 'b> PartialEq<Cow<'a, OsStr>> for PathBuf
impl<'a, 'b> PartialEq<Cow<'a, OsStr>> for Path
impl<'a, 'b> PartialEq<Cow<'a, Path>> for Path
impl<'a, 'b> PartialEq<Cow<'a, Path>> for &'b Path
impl<'a, 'b> PartialEq<Cow<'a, Path>> for PathBuf
impl<'a, 'b> PartialEq<Cow<'a, Path>> for OsStr
impl<'a, 'b> PartialEq<Cow<'a, Path>> for &'b OsStr
impl<'a, 'b> PartialEq<Cow<'a, Path>> for OsString
impl<'a, 'b> PartialEq<Cow<'a, str>> for &'b str
impl<'a, 'b> PartialEq<Cow<'a, str>> for String
impl<'a, 'b> PartialEq<Cow<'a, str>> for str
impl<'a, 'b, B, C> PartialEq<Cow<'b, C>> for Cow<'a, B> where    B: PartialEq<C> + ToOwned + ?Sized,    C: ToOwned + ?Sized,
impl<'a, 'b> PartialEq<Cow<'b, OsStr>> for &'a Path
impl<'a, 'b> PartialEq<OsStr> for Cow<'a, OsStr>
impl<'a, 'b> PartialEq<OsStr> for Cow<'a, Path>
impl<'a, 'b> PartialEq<OsString> for Cow<'a, OsStr>
impl<'a, 'b> PartialEq<OsString> for Cow<'a, Path>
impl<'a, 'b> PartialEq<Path> for Cow<'a, Path>
impl<'a, 'b> PartialEq<Path> for Cow<'a, OsStr>
impl<'a, 'b> PartialEq<PathBuf> for Cow<'a, Path>
impl<'a, 'b> PartialEq<PathBuf> for Cow<'a, OsStr>
impl<'a, 'b> PartialEq<String> for Cow<'a, str>
impl<'_, T, U, A> PartialEq<Vec<U, A>> for Cow<'_, [T]> where    A: Allocator,    T: PartialEq<U> + Clone,
impl<'a, 'b> PartialEq<str> for Cow<'a, str>
impl<'a, 'b> PartialOrd<&'a Path> for Cow<'b, OsStr>
impl<'a, 'b> PartialOrd<&'b OsStr> for Cow<'a, OsStr>
impl<'a, 'b> PartialOrd<&'b OsStr> for Cow<'a, Path>
impl<'a, 'b> PartialOrd<&'b Path> for Cow<'a, Path>
impl<'a, B> PartialOrd<Cow<'a, B>> for Cow<'a, B> where    B: PartialOrd<B> + ToOwned + ?Sized,
impl<'a, 'b> PartialOrd<Cow<'a, OsStr>> for OsStr
impl<'a, 'b> PartialOrd<Cow<'a, OsStr>> for &'b OsStr
impl<'a, 'b> PartialOrd<Cow<'a, OsStr>> for OsString
impl<'a, 'b> PartialOrd<Cow<'a, OsStr>> for PathBuf
impl<'a, 'b> PartialOrd<Cow<'a, OsStr>> for Path
impl<'a, 'b> PartialOrd<Cow<'a, Path>> for Path
impl<'a, 'b> PartialOrd<Cow<'a, Path>> for &'b Path
impl<'a, 'b> PartialOrd<Cow<'a, Path>> for PathBuf
impl<'a, 'b> PartialOrd<Cow<'a, Path>> for OsStr
impl<'a, 'b> PartialOrd<Cow<'a, Path>> for &'b OsStr
impl<'a, 'b> PartialOrd<Cow<'a, Path>> for OsString
impl<'a, 'b> PartialOrd<Cow<'b, OsStr>> for &'a Path
impl<'a, 'b> PartialOrd<OsStr> for Cow<'a, OsStr>
impl<'a, 'b> PartialOrd<OsStr> for Cow<'a, Path>
impl<'a, 'b> PartialOrd<OsString> for Cow<'a, OsStr>
impl<'a, 'b> PartialOrd<OsString> for Cow<'a, Path>
impl<'a, 'b> PartialOrd<Path> for Cow<'a, Path>
impl<'a, 'b> PartialOrd<Path> for Cow<'a, OsStr>
impl<'a, 'b> PartialOrd<PathBuf> for Cow<'a, Path>
impl<'a, 'b> PartialOrd<PathBuf> for Cow<'a, OsStr>
impl<'_> ToString for Cow<'_, str>
impl<T, A> AsMut<T> for Box<T, A> where    A: Allocator,    T: ?Sized,
impl<T, A> AsRef<T> for Box<T, A> where    A: Allocator,    T: ?Sized,
impl<T, A> Borrow<T> for Box<T, A> where    A: Allocator,    T: ?Sized,
impl<T, A> BorrowMut<T> for Box<T, A> where    A: Allocator,    T: ?Sized,
impl<B: BufRead + ?Sized> BufRead for Box<B>
impl<T, A> Clone for Box<[T], A> where    T: Clone,    A: Allocator + Clone,
impl Clone for Box<str, Global>
impl<T, A> Clone for Box<T, A> where    T: Clone,    A: Allocator + Clone,
impl Clone for Box<CStr>
impl Clone for Box<OsStr>
impl Clone for Box<Path>
impl<T, A> Debug for Box<T, A> where    T: Debug + ?Sized,    A: Allocator,
impl<T> Default for Box<[T], Global>
impl<T> Default for Box<T, Global> where    T: Default,
impl Default for Box<str, Global>
impl Default for Box<CStr>
impl Default for Box<OsStr>
impl<T, A> Deref for Box<T, A> where    A: Allocator,    T: ?Sized,
impl<T, A> DerefMut for Box<T, A> where    A: Allocator,    T: ?Sized,
impl<T, A> Display for Box<T, A> where    T: Display + ?Sized,    A: Allocator,
impl<I, A> DoubleEndedIterator for Box<I, A> where    I: DoubleEndedIterator + ?Sized,    A: Allocator,
impl<T, A> Drop for Box<T, A> where    A: Allocator,    T: ?Sized,
impl<T: Error> Error for Box<T>
impl<I, A> ExactSizeIterator for Box<I, A> where    I: ExactSizeIterator + ?Sized,    A: Allocator,
impl Extend<Box<str, Global>> for String
impl<Args, F, A> Fn<Args> for Box<F, A> where    F: Fn<Args> + ?Sized,    A: Allocator,
impl<Args, F, A> FnMut<Args> for Box<F, A> where    F: FnMut<Args> + ?Sized,    A: Allocator,
impl<Args, F, A> FnOnce<Args> for Box<F, A> where    F: FnOnce<Args> + ?Sized,    A: Allocator,
impl<'_, T> From<&'_ [T]> for Box<[T], Global> where    T: Copy,
impl From<&'_ CStr> for Box<CStr>
impl From<&'_ OsStr> for Box<OsStr>
impl From<&'_ Path> for Box<Path>
impl<'_> From<&'_ str> for Box<str, Global>
impl<'a> From<&'_ str> for Box<dyn Error + Send + Sync + 'a>
impl From<&'_ str> for Box<dyn Error>
impl<T, const N: usize> From<[T; N]> for Box<[T], Global>
impl<T, A> From<Box<[T], A>> for Vec<T, A> where    A: Allocator,
impl From<Box<CStr, Global>> for CString
impl From<Box<OsStr, Global>> for OsString
impl From<Box<Path, Global>> for PathBuf
impl<T, A> From<Box<T, A>> for Pin<Box<T, A>> where    A: Allocator + 'static,    T: ?Sized,
impl<T> From<Box<T, Global>> for Arc<T> where    T: ?Sized,
impl<T> From<Box<T, Global>> for Rc<T> where    T: ?Sized,
impl<A> From<Box<str, A>> for Box<[u8], A> where    A: Allocator,
impl From<Box<str, Global>> for String
impl From<CString> for Box<CStr>
impl<'_, T> From<Cow<'_, [T]>> for Box<[T], Global> where    T: Copy,
impl From<Cow<'_, CStr>> for Box<CStr>
impl From<Cow<'_, OsStr>> for Box<OsStr>
impl From<Cow<'_, Path>> for Box<Path>
impl<'_> From<Cow<'_, str>> for Box<str, Global>
impl<'a> From<Cow<'a, str>> for Box<dyn Error>
impl<'a, 'b> From<Cow<'b, str>> for Box<dyn Error + Send + Sync + 'a>
impl<'a, E: Error + 'a> From<E> for Box<dyn Error + 'a>
impl<'a, E: Error + Send + Sync + 'a> From<E> for Box<dyn Error + Send + Sync + 'a>
impl From<OsString> for Box<OsStr>
impl From<PathBuf> for Box<Path>
impl From<String> for Box<str, Global>
impl From<String> for Box<dyn Error + Send + Sync>
impl From<String> for Box<dyn Error>
impl<T> From<T> for Box<T, Global>
impl<T, A> From<Vec<T, A>> for Box<[T], A> where    A: Allocator,
impl FromIterator<Box<str, Global>> for String
impl<I> FromIterator<I> for Box<[I], Global>
impl<F, A> Future for Box<F, A> where    F: Future + Unpin + ?Sized,    A: Allocator + 'static,
impl<G, R, A> Generator<R> for Box<G, A> where    G: Generator<R> + Unpin + ?Sized,    A: Allocator + 'static,
impl<T, A> Hash for Box<T, A> where    T: Hash + ?Sized,    A: Allocator,
impl<T, A> Hasher for Box<T, A> where    T: Hasher + ?Sized,    A: Allocator,
impl<I, A> Iterator for Box<I, A> where    I: Iterator + ?Sized,    A: Allocator,
impl<T, A> Ord for Box<T, A> where    T: Ord + ?Sized,    A: Allocator,
impl<T, A> PartialEq<Box<T, A>> for Box<T, A> where    T: PartialEq<T> + ?Sized,    A: Allocator,
impl<T, A> PartialOrd<Box<T, A>> for Box<T, A> where    T: PartialOrd<T> + ?Sized,    A: Allocator,
impl<T, A> Pointer for Box<T, A> where    A: Allocator,    T: ?Sized,
impl<R: Read + ?Sized> Read for Box<R>
impl<S: Seek + ?Sized> Seek for Box<S>
impl<S> Stream for Box<S, Global> where    S: Stream + Unpin + ?Sized,
impl<T, const N: usize> TryFrom<Box<[T], Global>> for Box<[T; N], Global>
impl<W: Write + ?Sized> Write for Box<W>
impl Debug for BorrowError
impl Display for BorrowError
impl Error for BorrowError
impl Debug for BorrowMutError
impl Display for BorrowMutError
impl Error for BorrowMutError
impl<T> Clone for Cell<T> where    T: Copy,
impl<T> Debug for Cell<T> where    T: Copy + Debug,
impl<T> Default for Cell<T> where    T: Default,
impl<T> From<T> for Cell<T>
impl<T> Ord for Cell<T> where    T: Ord + Copy,
impl<T> PartialEq<Cell<T>> for Cell<T> where    T: PartialEq<T> + Copy,
impl<T> PartialOrd<Cell<T>> for Cell<T> where    T: PartialOrd<T> + Copy,
impl<'_, T> Debug for Ref<'_, T> where    T: Debug + ?Sized,
impl<'_, T> Deref for Ref<'_, T> where    T: ?Sized,
impl<'_, T> Display for Ref<'_, T> where    T: Display + ?Sized,
impl<T> Clone for RefCell<T> where    T: Clone,
impl<T> Debug for RefCell<T> where    T: Debug + ?Sized,
impl<T> Default for RefCell<T> where    T: Default,
impl<T> From<T> for RefCell<T>
impl<T> Ord for RefCell<T> where    T: Ord + ?Sized,
impl<T> PartialEq<RefCell<T>> for RefCell<T> where    T: PartialEq<T> + ?Sized,
impl<T> PartialOrd<RefCell<T>> for RefCell<T> where    T: PartialOrd<T> + ?Sized,
impl<'_, T> Debug for RefMut<'_, T> where    T: Debug + ?Sized,
impl<'_, T> Deref for RefMut<'_, T> where    T: ?Sized,
impl<'_, T> DerefMut for RefMut<'_, T> where    T: ?Sized,
impl<'_, T> Display for RefMut<'_, T> where    T: Display + ?Sized,
impl<T> Debug for UnsafeCell<T> where    T: ?Sized,
impl<T> Default for UnsafeCell<T> where    T: Default,
impl<T> From<T> for UnsafeCell<T>
impl Clone for CharTryFromError
impl Debug for CharTryFromError
impl Display for CharTryFromError
impl Error for CharTryFromError
impl PartialEq<CharTryFromError> for CharTryFromError
impl<I> Clone for DecodeUtf16<I> where    I: Clone + Iterator<Item = u16>,
impl<I> Debug for DecodeUtf16<I> where    I: Debug + Iterator<Item = u16>,
impl<I> Iterator for DecodeUtf16<I> where    I: Iterator<Item = u16>,
impl Clone for DecodeUtf16Error
impl Debug for DecodeUtf16Error
impl Display for DecodeUtf16Error
impl Error for DecodeUtf16Error
impl PartialEq<DecodeUtf16Error> for DecodeUtf16Error
impl Clone for EscapeDebug
impl Debug for EscapeDebug
impl Display for EscapeDebug
impl ExactSizeIterator for EscapeDebug
impl Iterator for EscapeDebug
impl Clone for EscapeUnicode
impl Debug for EscapeUnicode
impl Display for EscapeUnicode
impl ExactSizeIterator for EscapeUnicode
impl Iterator for EscapeUnicode
impl Clone for ToLowercase
impl Debug for ToLowercase
impl Display for ToLowercase
impl DoubleEndedIterator for ToLowercase
impl ExactSizeIterator for ToLowercase
impl Iterator for ToLowercase
impl Clone for ToUppercase
impl Debug for ToUppercase
impl Display for ToUppercase
impl DoubleEndedIterator for ToUppercase
impl ExactSizeIterator for ToUppercase
impl Iterator for ToUppercase
impl Clone for Ordering
impl Debug for Ordering
impl Hash for Ordering
impl Ord for Ordering
impl PartialEq<Ordering> for Ordering
impl PartialOrd<Ordering> for Ordering
impl<T> Clone for Reverse<T> where    T: Clone,
impl<T> Debug for Reverse<T> where    T: Debug,
impl<T> Default for Reverse<T> where    T: Default,
impl<T> Hash for Reverse<T> where    T: Hash,
impl<T> Ord for Reverse<T> where    T: Ord,
impl<T> PartialEq<Reverse<T>> for Reverse<T> where    T: PartialEq<T>,
impl<T> PartialOrd<Reverse<T>> for Reverse<T> where    T: PartialOrd<T>,
impl<K, V> Clone for BTreeMap<K, V> where    K: Clone,    V: Clone,
impl<K, V> Debug for BTreeMap<K, V> where    K: Debug,    V: Debug,
impl<K, V> Default for BTreeMap<K, V>
impl<K, V> Drop for BTreeMap<K, V>
impl<'a, K, V> Extend<(&'a K, &'a V)> for BTreeMap<K, V> where    K: Ord + Copy,    V: Copy,
impl<K, V> Extend<(K, V)> for BTreeMap<K, V> where    K: Ord,
impl<K, V, const N: usize> From<[(K, V); N]> for BTreeMap<K, V> where    K: Ord,
impl<K, V> FromIterator<(K, V)> for BTreeMap<K, V> where    K: Ord,
impl<K, V> Hash for BTreeMap<K, V> where    K: Hash,    V: Hash,
impl<'_, K, Q, V> Index<&'_ Q> for BTreeMap<K, V> where    K: Borrow<Q> + Ord,    Q: Ord + ?Sized,
impl<'a, K, V> IntoIterator for &'a mut BTreeMap<K, V>
impl<K, V> IntoIterator for BTreeMap<K, V>
impl<'a, K, V> IntoIterator for &'a BTreeMap<K, V>
impl<K, V> Ord for BTreeMap<K, V> where    K: Ord,    V: Ord,
impl<K, V> PartialEq<BTreeMap<K, V>> for BTreeMap<K, V> where    K: PartialEq<K>,    V: PartialEq<V>,
impl<K, V> PartialOrd<BTreeMap<K, V>> for BTreeMap<K, V> where    K: PartialOrd<K>,    V: PartialOrd<V>,
impl<'_, '_, T> BitAnd<&'_ BTreeSet<T>> for &'_ BTreeSet<T> where    T: Ord + Clone,
impl<'_, '_, T> BitOr<&'_ BTreeSet<T>> for &'_ BTreeSet<T> where    T: Ord + Clone,
impl<'_, '_, T> BitXor<&'_ BTreeSet<T>> for &'_ BTreeSet<T> where    T: Ord + Clone,
impl<T> Clone for BTreeSet<T> where    T: Clone,
impl<T> Debug for BTreeSet<T> where    T: Debug,
impl<T> Default for BTreeSet<T>
impl<'a, T> Extend<&'a T> for BTreeSet<T> where    T: 'a + Ord + Copy,
impl<T> Extend<T> for BTreeSet<T> where    T: Ord,
impl<T, const N: usize> From<[T; N]> for BTreeSet<T> where    T: Ord,
impl<T> FromIterator<T> for BTreeSet<T> where    T: Ord,
impl<T> Hash for BTreeSet<T> where    T: Hash,
impl<T> IntoIterator for BTreeSet<T>
impl<'a, T> IntoIterator for &'a BTreeSet<T>
impl<T> Ord for BTreeSet<T> where    T: Ord,
impl<T> PartialEq<BTreeSet<T>> for BTreeSet<T> where    T: PartialEq<T>,
impl<T> PartialOrd<BTreeSet<T>> for BTreeSet<T> where    T: PartialOrd<T>,
impl<'_, '_, T> Sub<&'_ BTreeSet<T>> for &'_ BTreeSet<T> where    T: Ord + Clone,
impl<T, S> BitAnd<&'_ HashSet<T, S>> for &HashSet<T, S> where    T: Eq + Hash + Clone,    S: BuildHasher + Default,
impl<T, S> BitOr<&'_ HashSet<T, S>> for &HashSet<T, S> where    T: Eq + Hash + Clone,    S: BuildHasher + Default,
impl<T, S> BitXor<&'_ HashSet<T, S>> for &HashSet<T, S> where    T: Eq + Hash + Clone,    S: BuildHasher + Default,
impl<T, S> Clone for HashSet<T, S> where    T: Clone,    S: Clone,
impl<T, S> Debug for HashSet<T, S> where    T: Debug,
impl<T, S> Default for HashSet<T, S> where    S: Default,
impl<'a, T, S> Extend<&'a T> for HashSet<T, S> where    T: 'a + Eq + Hash + Copy,    S: BuildHasher,
impl<T, S> Extend<T> for HashSet<T, S> where    T: Eq + Hash,    S: BuildHasher,
impl<T, const N: usize> From<[T; N]> for HashSet<T, RandomState> where    T: Eq + Hash,
impl<T, S> FromIterator<T> for HashSet<T, S> where    T: Eq + Hash,    S: BuildHasher + Default,
impl<'a, T, S> IntoIterator for &'a HashSet<T, S>
impl<T, S> IntoIterator for HashSet<T, S>
impl<T, S> PartialEq<HashSet<T, S>> for HashSet<T, S> where    T: Eq + Hash,    S: BuildHasher,
impl<T, S> Sub<&'_ HashSet<T, S>> for &HashSet<T, S> where    T: Eq + Hash + Clone,    S: BuildHasher + Default,
impl<T> Clone for LinkedList<T> where    T: Clone,
impl<T> Debug for LinkedList<T> where    T: Debug,
impl<T> Default for LinkedList<T>
impl<T> Drop for LinkedList<T>
impl<'a, T> Extend<&'a T> for LinkedList<T> where    T: 'a + Copy,
impl<T> Extend<T> for LinkedList<T>
impl<T, const N: usize> From<[T; N]> for LinkedList<T>
impl<T> FromIterator<T> for LinkedList<T>
impl<T> Hash for LinkedList<T> where    T: Hash,
impl<T> IntoIterator for LinkedList<T>
impl<'a, T> IntoIterator for &'a LinkedList<T>
impl<'a, T> IntoIterator for &'a mut LinkedList<T>
impl<T> Ord for LinkedList<T> where    T: Ord,
impl<T> PartialEq<LinkedList<T>> for LinkedList<T> where    T: PartialEq<T>,
impl<T> PartialOrd<LinkedList<T>> for LinkedList<T> where    T: PartialOrd<T>,
impl<T, A> Clone for VecDeque<T, A> where    T: Clone,    A: Allocator + Clone,
impl<T, A> Debug for VecDeque<T, A> where    T: Debug,    A: Allocator,
impl<T> Default for VecDeque<T, Global>
impl<T, A> Drop for VecDeque<T, A> where    A: Allocator,
impl<'a, T, A> Extend<&'a T> for VecDeque<T, A> where    T: 'a + Copy,    A: Allocator,
impl<T, A> Extend<T> for VecDeque<T, A> where    A: Allocator,
impl<T, const N: usize> From<[T; N]> for VecDeque<T, Global>
impl<T, A> From<Vec<T, A>> for VecDeque<T, A> where    A: Allocator,
impl<T, A> From<VecDeque<T, A>> for Vec<T, A> where    A: Allocator,
impl<T> FromIterator<T> for VecDeque<T, Global>
impl<T, A> Hash for VecDeque<T, A> where    T: Hash,    A: Allocator,
impl<T, A> Index<usize> for VecDeque<T, A> where    A: Allocator,
impl<T, A> IndexMut<usize> for VecDeque<T, A> where    A: Allocator,
impl<'a, T, A> IntoIterator for &'a VecDeque<T, A> where    A: Allocator,
impl<T, A> IntoIterator for VecDeque<T, A> where    A: Allocator,
impl<'a, T, A> IntoIterator for &'a mut VecDeque<T, A> where    A: Allocator,
impl<T, A> Ord for VecDeque<T, A> where    T: Ord,    A: Allocator,
impl<'_, T, U, A, const N: usize> PartialEq<&'_ [U; N]> for VecDeque<T, A> where    A: Allocator,    T: PartialEq<U>,
impl<'_, T, U, A> PartialEq<&'_ [U]> for VecDeque<T, A> where    A: Allocator,    T: PartialEq<U>,
impl<'_, T, U, A, const N: usize> PartialEq<&'_ mut [U; N]> for VecDeque<T, A> where    A: Allocator,    T: PartialEq<U>,
impl<'_, T, U, A> PartialEq<&'_ mut [U]> for VecDeque<T, A> where    A: Allocator,    T: PartialEq<U>,
impl<T, U, A, const N: usize> PartialEq<[U; N]> for VecDeque<T, A> where    A: Allocator,    T: PartialEq<U>,
impl<T, U, A> PartialEq<Vec<U, A>> for VecDeque<T, A> where    A: Allocator,    T: PartialEq<U>,
impl<T, A> PartialEq<VecDeque<T, A>> for VecDeque<T, A> where    T: PartialEq<T>,    A: Allocator,
impl<T, A> PartialOrd<VecDeque<T, A>> for VecDeque<T, A> where    T: PartialOrd<T>,    A: Allocator,
impl<T> Clone for BinaryHeap<T> where    T: Clone,
impl<T> Debug for BinaryHeap<T> where    T: Debug,
impl<T> Default for BinaryHeap<T> where    T: Ord,
impl<'a, T> Extend<&'a T> for BinaryHeap<T> where    T: 'a + Ord + Copy,
impl<T> Extend<T> for BinaryHeap<T> where    T: Ord,
impl<T, const N: usize> From<[T; N]> for BinaryHeap<T> where    T: Ord,
impl<T> From<BinaryHeap<T>> for Vec<T, Global>
impl<T> From<Vec<T, Global>> for BinaryHeap<T> where    T: Ord,
impl<T> FromIterator<T> for BinaryHeap<T> where    T: Ord,
impl<T> IntoIterator for BinaryHeap<T>
impl<'a, T> IntoIterator for &'a BinaryHeap<T>
impl<'a, T> Debug for Drain<'a, T> where    T: 'a + Debug,
impl<'_, T> DoubleEndedIterator for Drain<'_, T>
impl<'_, T> ExactSizeIterator for Drain<'_, T>
impl<'_, T> Iterator for Drain<'_, T>
impl<T> Clone for IntoIter<T> where    T: Clone,
impl<T> Debug for IntoIter<T> where    T: Debug,
impl<T> DoubleEndedIterator for IntoIter<T>
impl<T> ExactSizeIterator for IntoIter<T>
impl<T> Iterator for IntoIter<T>
impl<'_, T> Clone for Iter<'_, T>
impl<'_, T> Debug for Iter<'_, T> where    T: Debug,
impl<'a, T> DoubleEndedIterator for Iter<'a, T>
impl<'_, T> ExactSizeIterator for Iter<'_, T>
impl<'a, T> Iterator for Iter<'a, T>
impl<'_, T> Debug for PeekMut<'_, T> where    T: Ord + Debug,
impl<'_, T> Deref for PeekMut<'_, T> where    T: Ord,
impl<'_, T> DerefMut for PeekMut<'_, T> where    T: Ord,
impl<'_, T> Drop for PeekMut<'_, T> where    T: Ord,
impl<'_, K, V> Clone for Keys<'_, K, V>
impl<'_, K, V> Debug for Keys<'_, K, V> where    K: Debug,
impl<'a, K, V> DoubleEndedIterator for Keys<'a, K, V>
impl<'_, K, V> ExactSizeIterator for Keys<'_, K, V>
impl<'a, K, V> Iterator for Keys<'a, K, V>
impl<'_, K, V> Clone for Values<'_, K, V>
impl<'_, K, V> Debug for Values<'_, K, V> where    V: Debug,
impl<'a, K, V> DoubleEndedIterator for Values<'a, K, V>
impl<'_, K, V> ExactSizeIterator for Values<'_, K, V>
impl<'a, K, V> Iterator for Values<'a, K, V>
impl<K, V, S> Clone for HashMap<K, V, S> where    K: Clone,    V: Clone,    S: Clone,
impl<K, V, S> Debug for HashMap<K, V, S> where    K: Debug,    V: Debug,
impl<K, V, S> Default for HashMap<K, V, S> where    S: Default,
impl<'a, K, V, S> Extend<(&'a K, &'a V)> for HashMap<K, V, S> where    K: Eq + Hash + Copy,    V: Copy,    S: BuildHasher,
impl<K, V, S> Extend<(K, V)> for HashMap<K, V, S> where    K: Eq + Hash,    S: BuildHasher,
impl<K, V, const N: usize> From<[(K, V); N]> for HashMap<K, V, RandomState> where    K: Eq + Hash,
impl<K, V, S> FromIterator<(K, V)> for HashMap<K, V, S> where    K: Eq + Hash,    S: BuildHasher + Default,
impl<K, Q: ?Sized, V, S> Index<&'_ Q> for HashMap<K, V, S> where    K: Eq + Hash + Borrow<Q>,    Q: Eq + Hash,    S: BuildHasher,
impl<'a, K, V, S> IntoIterator for &'a HashMap<K, V, S>
impl<'a, K, V, S> IntoIterator for &'a mut HashMap<K, V, S>
impl<K, V, S> IntoIterator for HashMap<K, V, S>
impl<K, V, S> PartialEq<HashMap<K, V, S>> for HashMap<K, V, S> where    K: Eq + Hash,    V: PartialEq,    S: BuildHasher,
impl Clone for Infallible
impl Debug for Infallible
impl Display for Infallible
impl Error for Infallible
impl From<!> for Infallible
impl From<Infallible> for TryFromSliceError
impl From<Infallible> for TryFromIntError
impl Hash for Infallible
impl Ord for Infallible
impl PartialEq<Infallible> for Infallible
impl PartialOrd<Infallible> for Infallible
impl Debug for Args
impl DoubleEndedIterator for Args
impl ExactSizeIterator for Args
impl Iterator for Args
impl Debug for ArgsOs
impl DoubleEndedIterator for ArgsOs
impl ExactSizeIterator for ArgsOs
impl Iterator for ArgsOs
impl Debug for JoinPathsError
impl Display for JoinPathsError
impl Error for JoinPathsError
impl Debug for SplitPaths<'_>
impl<'a> Iterator for SplitPaths<'a>
impl Debug for Vars
impl Iterator for Vars
impl Debug for VarsOs
impl Iterator for VarsOs
impl Debug for Alignment
impl<'a> Clone for Arguments<'a>
impl<'_> Debug for Arguments<'_>
impl<'_> Display for Arguments<'_>
impl Clone for Error
impl Debug for Error
impl Default for Error
impl Display for Error
impl Error for Error
impl Hash for Error
impl Ord for Error
impl PartialEq<Error> for Error
impl PartialOrd<Error> for Error
impl<'_> Write for Formatter<'_>
impl Debug for DirBuilder
impl DirBuilderExt for DirBuilder
impl Debug for DirEntry
impl DirEntryExt for DirEntry
impl DirEntryExt for DirEntry
impl DirEntryExt2 for DirEntry
impl AsFd for File
impl AsHandle for File
impl AsRawFd for File
impl AsRawHandle for File
impl Debug for File
impl FileExt for File
impl FileExt for File
impl FileExt for File
impl From<File> for OwnedHandle
impl From<File> for OwnedFd
impl From<File> for Stdio
impl From<OwnedFd> for File
impl From<OwnedHandle> for File
impl FromRawFd for File
impl FromRawHandle for File
impl IntoRawFd for File
impl IntoRawHandle for File
impl Read for File
impl Read for &File
impl Seek for File
impl Seek for &File
impl Write for File
impl Write for &File
impl Clone for FileType
impl Debug for FileType
impl FileTypeExt for FileType
impl FileTypeExt for FileType
impl FileTypeExt for FileType
impl Hash for FileType
impl PartialEq<FileType> for FileType
impl Clone for Metadata
impl Debug for Metadata
impl MetadataExt for Metadata
impl MetadataExt for Metadata
impl MetadataExt for Metadata
impl MetadataExt for Metadata
impl Clone for Permissions
impl Debug for Permissions
impl PartialEq<Permissions> for Permissions
impl PermissionsExt for Permissions
impl Debug for ReadDir
impl Iterator for ReadDir
impl<T> Clone for Pending<T>
impl<T> Debug for Pending<T>
impl<T> Future for Pending<T>
impl<T> Clone for Ready<T> where    T: Clone,
impl<T> Debug for Ready<T> where    T: Debug,
impl<T> Future for Ready<T>
impl<H> BuildHasher for BuildHasherDefault<H> where    H: Default + Hasher,
impl<H> Clone for BuildHasherDefault<H>
impl<H> Debug for BuildHasherDefault<H>
impl<H> Default for BuildHasherDefault<H>
impl<H> PartialEq<BuildHasherDefault<H>> for BuildHasherDefault<H>
impl<R: Read> BufRead for BufReader<R>
impl<R> Debug for BufReader<R> where    R: Debug,
impl<R: Read> Read for BufReader<R>
impl<R: Seek> Seek for BufReader<R>
impl<W: Write> Debug for BufWriter<W> where    W: Debug,
impl<W: Write> Drop for BufWriter<W>
impl<W: Write + Seek> Seek for BufWriter<W>
impl<W: Write> Write for BufWriter<W>
impl<R: Debug> Debug for Bytes<R>
impl<R: Read> Iterator for Bytes<R>
impl<T: BufRead, U: BufRead> BufRead for Chain<T, U>
impl<T: Debug, U: Debug> Debug for Chain<T, U>
impl<T: Read, U: Read> Read for Chain<T, U>
impl<T> BufRead for Cursor<T> where    T: AsRef<[u8]>,
impl<T> Clone for Cursor<T> where    T: Clone,
impl<T: Debug> Debug for Cursor<T>
impl<T: Default> Default for Cursor<T>
impl<T: PartialEq> PartialEq<Cursor<T>> for Cursor<T>
impl<T> Read for Cursor<T> where    T: AsRef<[u8]>,
impl<T> Seek for Cursor<T> where    T: AsRef<[u8]>,
impl Write for Cursor<&mut [u8]>
impl Write for Cursor<&mut Vec<u8>>
impl Write for Cursor<Vec<u8>>
impl Write for Cursor<Box<[u8]>>
impl BufRead for Empty
impl Clone for Empty
impl Debug for Empty
impl Default for Empty
impl Read for Empty
impl Seek for Empty
impl Debug for Error
impl Display for Error
impl Error for Error
impl From<ErrorKind> for Error
impl<W> From<IntoInnerError<W>> for Error
impl From<NulError> for Error
impl<'a> Clone for IoSlice<'a>
impl<'a> Debug for IoSlice<'a>
impl<'a> Deref for IoSlice<'a>
impl<'a> Debug for IoSliceMut<'a>
impl<'a> Deref for IoSliceMut<'a>
impl<'a> DerefMut for IoSliceMut<'a>
impl<W: Write> Debug for LineWriter<W> where    W: Debug,
impl<W: Write> Write for LineWriter<W>
impl Clone for SeekFrom
impl Debug for SeekFrom
impl PartialEq<SeekFrom> for SeekFrom
impl Clone for Sink
impl Debug for Sink
impl Default for Sink
impl Write for Sink
impl Write for &Sink
impl AsFd for Stderr
impl AsHandle for Stderr
impl AsRawFd for Stderr
impl AsRawHandle for Stderr
impl Debug for Stderr
impl Write for Stderr
impl Write for &Stderr
impl<'a> AsFd for StderrLock<'a>
impl<'a> AsHandle for StderrLock<'a>
impl<'a> AsRawFd for StderrLock<'a>
impl<'a> AsRawHandle for StderrLock<'a>
impl Debug for StderrLock<'_>
impl Write for StderrLock<'_>
impl AsFd for Stdin
impl AsHandle for Stdin
impl AsRawFd for Stdin
impl AsRawHandle for Stdin
impl Debug for Stdin
impl Read for Stdin
impl<'a> AsFd for StdinLock<'a>
impl<'a> AsHandle for StdinLock<'a>
impl<'a> AsRawFd for StdinLock<'a>
impl<'a> AsRawHandle for StdinLock<'a>
impl BufRead for StdinLock<'_>
impl Debug for StdinLock<'_>
impl Read for StdinLock<'_>
impl AsFd for Stdout
impl AsHandle for Stdout
impl AsRawFd for Stdout
impl AsRawHandle for Stdout
impl Debug for Stdout
impl Write for Stdout
impl Write for &Stdout
impl<'a> AsFd for StdoutLock<'a>
impl<'a> AsHandle for StdoutLock<'a>
impl<'a> AsRawFd for StdoutLock<'a>
impl<'a> AsRawHandle for StdoutLock<'a>
impl Debug for StdoutLock<'_>
impl Write for StdoutLock<'_>
impl<T: BufRead> BufRead for Take<T>
impl<T: Debug> Debug for Take<T>
impl<T: Read> Read for Take<T>
impl<A, B> Clone for Chain<A, B> where    A: Clone,    B: Clone,
impl<A, B> Debug for Chain<A, B> where    A: Debug,    B: Debug,
impl<A, B> DoubleEndedIterator for Chain<A, B> where    A: DoubleEndedIterator,    B: DoubleEndedIterator<Item = <A as Iterator>::Item>,
impl<A, B> Iterator for Chain<A, B> where    A: Iterator,    B: Iterator<Item = <A as Iterator>::Item>,
impl<I> Clone for Cloned<I> where    I: Clone,
impl<I> Debug for Cloned<I> where    I: Debug,
impl<'a, I, T> DoubleEndedIterator for Cloned<I> where    T: 'a + Clone,    I: DoubleEndedIterator<Item = &'a T>,
impl<'a, I, T> ExactSizeIterator for Cloned<I> where    T: 'a + Clone,    I: ExactSizeIterator<Item = &'a T>,
impl<'a, I, T> Iterator for Cloned<I> where    T: 'a + Clone,    I: Iterator<Item = &'a T>,
impl<T, F> Clone for Successors<T, F> where    T: Clone,    F: Clone,
impl<T, F> Debug for Successors<T, F> where    T: Debug,
impl<T, F> Iterator for Successors<T, F> where    F: FnMut(&T) -> Option<T>,
impl<T> Clone for PhantomData<T> where    T: ?Sized,
impl<T> Debug for PhantomData<T> where    T: ?Sized,
impl<T> Default for PhantomData<T> where    T: ?Sized,
impl<T> Hash for PhantomData<T> where    T: ?Sized,
impl<T> Ord for PhantomData<T> where    T: ?Sized,
impl<T> PartialEq<PhantomData<T>> for PhantomData<T> where    T: ?Sized,
impl<T> PartialOrd<PhantomData<T>> for PhantomData<T> where    T: ?Sized,
impl Clone for PhantomPinned
impl Debug for PhantomPinned
impl Default for PhantomPinned
impl Hash for PhantomPinned
impl Ord for PhantomPinned
impl PartialEq<PhantomPinned> for PhantomPinned
impl PartialOrd<PhantomPinned> for PhantomPinned
impl<T> Clone for Discriminant<T>
impl<T> Debug for Discriminant<T>
impl<T> Hash for Discriminant<T>
impl<T> PartialEq<Discriminant<T>> for Discriminant<T>
impl<T> Clone for ManuallyDrop<T> where    T: Clone + ?Sized,
impl<T> Debug for ManuallyDrop<T> where    T: Debug + ?Sized,
impl<T> Default for ManuallyDrop<T> where    T: Default + ?Sized,
impl<T> Deref for ManuallyDrop<T> where    T: ?Sized,
impl<T> DerefMut for ManuallyDrop<T> where    T: ?Sized,
impl<T> Hash for ManuallyDrop<T> where    T: Hash + ?Sized,
impl<T> Ord for ManuallyDrop<T> where    T: Ord + ?Sized,
impl<T> PartialEq<ManuallyDrop<T>> for ManuallyDrop<T> where    T: PartialEq<T> + ?Sized,
impl<T> PartialOrd<ManuallyDrop<T>> for ManuallyDrop<T> where    T: PartialOrd<T> + ?Sized,
impl<'a> Debug for Incoming<'a>
impl<'a> Iterator for Incoming<'a>
impl Clone for IpAddr
impl Debug for IpAddr
impl Display for IpAddr
impl From<[u16; 8]> for IpAddr
impl From<[u8; 16]> for IpAddr
impl From<[u8; 4]> for IpAddr
impl From<Ipv4Addr> for IpAddr
impl From<Ipv6Addr> for IpAddr
impl FromStr for IpAddr
impl Hash for IpAddr
impl Ord for IpAddr
impl PartialEq<IpAddr> for IpAddr
impl PartialEq<IpAddr> for Ipv4Addr
impl PartialEq<IpAddr> for Ipv6Addr
impl PartialEq<Ipv4Addr> for IpAddr
impl PartialEq<Ipv6Addr> for IpAddr
impl PartialOrd<IpAddr> for IpAddr
impl PartialOrd<IpAddr> for Ipv4Addr
impl PartialOrd<IpAddr> for Ipv6Addr
impl PartialOrd<Ipv4Addr> for IpAddr
impl PartialOrd<Ipv6Addr> for IpAddr
impl Clone for Ipv4Addr
impl Debug for Ipv4Addr
impl Display for Ipv4Addr
impl From<[u8; 4]> for Ipv4Addr
impl From<Ipv4Addr> for IpAddr
impl From<Ipv4Addr> for u32
impl From<u32> for Ipv4Addr
impl FromStr for Ipv4Addr
impl Hash for Ipv4Addr
impl Ord for Ipv4Addr
impl PartialEq<IpAddr> for Ipv4Addr
impl PartialEq<Ipv4Addr> for Ipv4Addr
impl PartialEq<Ipv4Addr> for IpAddr
impl PartialOrd<IpAddr> for Ipv4Addr
impl PartialOrd<Ipv4Addr> for Ipv4Addr
impl PartialOrd<Ipv4Addr> for IpAddr
impl Clone for Ipv6Addr
impl Debug for Ipv6Addr
impl Display for Ipv6Addr
impl From<[u16; 8]> for Ipv6Addr
impl From<[u8; 16]> for Ipv6Addr
impl From<Ipv6Addr> for IpAddr
impl From<Ipv6Addr> for u128
impl From<u128> for Ipv6Addr
impl FromStr for Ipv6Addr
impl Hash for Ipv6Addr
impl Ord for Ipv6Addr
impl PartialEq<IpAddr> for Ipv6Addr
impl PartialEq<Ipv6Addr> for Ipv6Addr
impl PartialEq<Ipv6Addr> for IpAddr
impl PartialOrd<IpAddr> for Ipv6Addr
impl PartialOrd<Ipv6Addr> for Ipv6Addr
impl PartialOrd<Ipv6Addr> for IpAddr
impl Clone for Shutdown
impl Debug for Shutdown
impl PartialEq<Shutdown> for Shutdown
impl Clone for SocketAddr
impl Debug for SocketAddr
impl Display for SocketAddr
impl<I: Into<IpAddr>> From<(I, u16)> for SocketAddr
impl From<SocketAddrV4> for SocketAddr
impl From<SocketAddrV6> for SocketAddr
impl FromStr for SocketAddr
impl Hash for SocketAddr
impl Ord for SocketAddr
impl PartialEq<SocketAddr> for SocketAddr
impl PartialOrd<SocketAddr> for SocketAddr
impl ToSocketAddrs for SocketAddr
impl Clone for SocketAddrV4
impl Debug for SocketAddrV4
impl Display for SocketAddrV4
impl From<SocketAddrV4> for SocketAddr
impl FromStr for SocketAddrV4
impl Hash for SocketAddrV4
impl Ord for SocketAddrV4
impl PartialEq<SocketAddrV4> for SocketAddrV4
impl PartialOrd<SocketAddrV4> for SocketAddrV4
impl ToSocketAddrs for SocketAddrV4
impl Clone for SocketAddrV6
impl Debug for SocketAddrV6
impl Display for SocketAddrV6
impl From<SocketAddrV6> for SocketAddr
impl FromStr for SocketAddrV6
impl Hash for SocketAddrV6
impl Ord for SocketAddrV6
impl PartialEq<SocketAddrV6> for SocketAddrV6
impl PartialOrd<SocketAddrV6> for SocketAddrV6
impl ToSocketAddrs for SocketAddrV6
impl AsFd for TcpListener
impl AsRawFd for TcpListener
impl AsRawSocket for TcpListener
impl AsSocket for TcpListener
impl Debug for TcpListener
impl From<OwnedFd> for TcpListener
impl From<OwnedSocket> for TcpListener
impl From<TcpListener> for OwnedSocket
impl From<TcpListener> for OwnedFd
impl FromRawFd for TcpListener
impl FromRawSocket for TcpListener
impl IntoRawFd for TcpListener
impl IntoRawSocket for TcpListener
impl AsFd for TcpStream
impl AsRawFd for TcpStream
impl AsRawSocket for TcpStream
impl AsSocket for TcpStream
impl Debug for TcpStream
impl From<OwnedFd> for TcpStream
impl From<OwnedSocket> for TcpStream
impl From<TcpStream> for OwnedSocket
impl From<TcpStream> for OwnedFd
impl FromRawFd for TcpStream
impl FromRawSocket for TcpStream
impl IntoRawFd for TcpStream
impl IntoRawSocket for TcpStream
impl Read for TcpStream
impl Read for &TcpStream
impl Write for TcpStream
impl Write for &TcpStream
impl AsFd for UdpSocket
impl AsRawFd for UdpSocket
impl AsRawSocket for UdpSocket
impl AsSocket for UdpSocket
impl Debug for UdpSocket
impl From<OwnedFd> for UdpSocket
impl From<OwnedSocket> for UdpSocket
impl From<UdpSocket> for OwnedSocket
impl From<UdpSocket> for OwnedFd
impl FromRawFd for UdpSocket
impl FromRawSocket for UdpSocket
impl IntoRawFd for UdpSocket
impl IntoRawSocket for UdpSocket
impl Clone for FpCategory
impl Debug for FpCategory
impl PartialEq<FpCategory> for FpCategory
impl<T> Clone for Bound<T> where    T: Clone,
impl<T> Debug for Bound<T> where    T: Debug,
impl<T> Hash for Bound<T> where    T: Hash,
impl<T> PartialEq<Bound<T>> for Bound<T> where    T: PartialEq<T>,
impl<B, C> Clone for ControlFlow<B, C> where    B: Clone,    C: Clone,
impl<B, C> Debug for ControlFlow<B, C> where    B: Debug,    C: Debug,
impl<B, C> FromResidual<<ControlFlow<B, C> as Try>::Residual> for ControlFlow<B, C>
impl<B, C> PartialEq<ControlFlow<B, C>> for ControlFlow<B, C> where    B: PartialEq<B>,    C: PartialEq<C>,
impl<B, C> Residual<C> for ControlFlow<B, Infallible>
impl<B, C> Try for ControlFlow<B, C>
impl<Idx> Clone for Range<Idx> where    Idx: Clone,
impl<Idx> Debug for Range<Idx> where    Idx: Debug,
impl<Idx> Default for Range<Idx> where    Idx: Default,
impl<A> DoubleEndedIterator for Range<A> where    A: Step,
impl ExactSizeIterator for Range<isize>
impl ExactSizeIterator for Range<i32>
impl ExactSizeIterator for Range<u8>
impl ExactSizeIterator for Range<u32>
impl ExactSizeIterator for Range<i8>
impl ExactSizeIterator for Range<i16>
impl ExactSizeIterator for Range<usize>
impl ExactSizeIterator for Range<u16>
impl<Idx> Hash for Range<Idx> where    Idx: Hash,
impl Index<Range<usize>> for String
impl IndexMut<Range<usize>> for String
impl<A> Iterator for Range<A> where    A: Step,
impl<Idx> PartialEq<Range<Idx>> for Range<Idx> where    Idx: PartialEq<Idx>,
impl<T> RangeBounds<T> for Range<T>
impl<'_, T> RangeBounds<T> for Range<&'_ T>
impl<T> SliceIndex<[T]> for Range<usize>
impl SliceIndex<str> for Range<usize>
impl<Idx> Clone for RangeFrom<Idx> where    Idx: Clone,
impl<Idx> Debug for RangeFrom<Idx> where    Idx: Debug,
impl<Idx> Hash for RangeFrom<Idx> where    Idx: Hash,
impl Index<RangeFrom<usize>> for String
impl Index<RangeFrom<usize>> for CStr
impl IndexMut<RangeFrom<usize>> for String
impl<A> Iterator for RangeFrom<A> where    A: Step,
impl<Idx> PartialEq<RangeFrom<Idx>> for RangeFrom<Idx> where    Idx: PartialEq<Idx>,
impl<'_, T> RangeBounds<T> for RangeFrom<&'_ T>
impl<T> RangeBounds<T> for RangeFrom<T>
impl<T> SliceIndex<[T]> for RangeFrom<usize>
impl SliceIndex<str> for RangeFrom<usize>
impl Clone for RangeFull
impl Debug for RangeFull
impl Default for RangeFull
impl Hash for RangeFull
impl Index<RangeFull> for String
impl Index<RangeFull> for CString
impl Index<RangeFull> for OsString
impl IndexMut<RangeFull> for String
impl IndexMut<RangeFull> for OsString
impl PartialEq<RangeFull> for RangeFull
impl<T> RangeBounds<T> for RangeFull where    T: ?Sized,
impl<T> SliceIndex<[T]> for RangeFull
impl SliceIndex<str> for RangeFull
impl<Idx> Clone for RangeInclusive<Idx> where    Idx: Clone,
impl<Idx> Debug for RangeInclusive<Idx> where    Idx: Debug,
impl<A> DoubleEndedIterator for RangeInclusive<A> where    A: Step,
impl ExactSizeIterator for RangeInclusive<u8>
impl ExactSizeIterator for RangeInclusive<i8>
impl ExactSizeIterator for RangeInclusive<i16>
impl ExactSizeIterator for RangeInclusive<u16>
impl<Idx> Hash for RangeInclusive<Idx> where    Idx: Hash,
impl Index<RangeInclusive<usize>> for String
impl IndexMut<RangeInclusive<usize>> for String
impl<A> Iterator for RangeInclusive<A> where    A: Step,
impl<Idx> PartialEq<RangeInclusive<Idx>> for RangeInclusive<Idx> where    Idx: PartialEq<Idx>,
impl<'_, T> RangeBounds<T> for RangeInclusive<&'_ T>
impl<T> RangeBounds<T> for RangeInclusive<T>
impl<T> SliceIndex<[T]> for RangeInclusive<usize>
impl SliceIndex<str> for RangeInclusive<usize>
impl<Idx> Clone for RangeTo<Idx> where    Idx: Clone,
impl<Idx> Debug for RangeTo<Idx> where    Idx: Debug,
impl<Idx> Hash for RangeTo<Idx> where    Idx: Hash,
impl Index<RangeTo<usize>> for String
impl IndexMut<RangeTo<usize>> for String
impl<Idx> PartialEq<RangeTo<Idx>> for RangeTo<Idx> where    Idx: PartialEq<Idx>,
impl<T> RangeBounds<T> for RangeTo<T>
impl<'_, T> RangeBounds<T> for RangeTo<&'_ T>
impl<T> SliceIndex<[T]> for RangeTo<usize>
impl SliceIndex<str> for RangeTo<usize>
impl<Idx> Clone for RangeToInclusive<Idx> where    Idx: Clone,
impl<Idx> Debug for RangeToInclusive<Idx> where    Idx: Debug,
impl<Idx> Hash for RangeToInclusive<Idx> where    Idx: Hash,
impl Index<RangeToInclusive<usize>> for String
impl IndexMut<RangeToInclusive<usize>> for String
impl<Idx> PartialEq<RangeToInclusive<Idx>> for RangeToInclusive<Idx> where    Idx: PartialEq<Idx>,
impl<'_, T> RangeBounds<T> for RangeToInclusive<&'_ T>
impl<T> RangeBounds<T> for RangeToInclusive<T>
impl<T> SliceIndex<[T]> for RangeToInclusive<usize>
impl SliceIndex<str> for RangeToInclusive<usize>
impl<'_, A> Clone for Iter<'_, A>
impl<'a, A> Debug for Iter<'a, A> where    A: 'a + Debug,
impl<'a, A> DoubleEndedIterator for Iter<'a, A>
impl<'_, A> ExactSizeIterator for Iter<'_, A>
impl<'a, A> Iterator for Iter<'a, A>
impl<'a> Clone for Location<'a>
impl<'a> Debug for Location<'a>
impl<'_> Display for Location<'_>
impl<'a> Hash for Location<'a>
impl<'a> Ord for Location<'a>
impl<'a> PartialEq<Location<'a>> for Location<'a>
impl<'a> PartialOrd<Location<'a>> for Location<'a>
impl<'a> Clone for Ancestors<'a>
impl<'a> Debug for Ancestors<'a>
impl<'a> Iterator for Ancestors<'a>
impl AsRef<OsStr> for Component<'_>
impl AsRef<Path> for Component<'_>
impl<'a> Clone for Component<'a>
impl<'a> Debug for Component<'a>
impl<'a> Hash for Component<'a>
impl<'a> Ord for Component<'a>
impl<'a> PartialEq<Component<'a>> for Component<'a>
impl<'a> PartialOrd<Component<'a>> for Component<'a>
impl AsRef<OsStr> for Components<'_>
impl AsRef<Path> for Components<'_>
impl<'a> Clone for Components<'a>
impl Debug for Components<'_>
impl<'a> DoubleEndedIterator for Components<'a>
impl<'a> Iterator for Components<'a>
impl Ord for Components<'_>
impl<'a> PartialEq<Components<'a>> for Components<'a>
impl<'a> PartialOrd<Components<'a>> for Components<'a>
impl Debug for Display<'_>
impl Display for Display<'_>
impl AsRef<OsStr> for Iter<'_>
impl AsRef<Path> for Iter<'_>
impl<'a> Clone for Iter<'a>
impl Debug for Iter<'_>
impl<'a> DoubleEndedIterator for Iter<'a>
impl<'a> Iterator for Iter<'a>
impl AsRef<OsStr> for Path
impl AsRef<Path> for Component<'_>
impl AsRef<Path> for Components<'_>
impl AsRef<Path> for Iter<'_>
impl AsRef<Path> for Path
impl AsRef<Path> for OsStr
impl AsRef<Path> for Cow<'_, OsStr>
impl AsRef<Path> for OsString
impl AsRef<Path> for str
impl AsRef<Path> for String
impl AsRef<Path> for PathBuf
impl Borrow<Path> for PathBuf
impl Debug for Path
impl From<&'_ Path> for Box<Path>
impl From<&'_ Path> for Arc<Path>
impl From<&'_ Path> for Rc<Path>
impl<'a> From<&'a Path> for Cow<'a, Path>
impl Hash for Path
impl<'a> IntoIterator for &'a Path
impl Ord for Path
impl<'a, 'b> PartialEq<&'a OsStr> for Path
impl<'a, 'b> PartialEq<&'a Path> for PathBuf
impl<'a, 'b> PartialEq<&'a Path> for OsStr
impl<'a, 'b> PartialEq<&'a Path> for Cow<'b, OsStr>
impl<'a, 'b> PartialEq<&'a Path> for OsString
impl<'a, 'b> PartialEq<&'b Path> for Cow<'a, Path>
impl<'a, 'b> PartialEq<Cow<'a, OsStr>> for Path
impl<'a, 'b> PartialEq<Cow<'a, Path>> for Path
impl<'a, 'b> PartialEq<Cow<'a, Path>> for &'b Path
impl<'a, 'b> PartialEq<Cow<'b, OsStr>> for &'a Path
impl<'a, 'b> PartialEq<OsStr> for Path
impl<'a, 'b> PartialEq<OsStr> for &'a Path
impl<'a, 'b> PartialEq<OsString> for Path
impl<'a, 'b> PartialEq<OsString> for &'a Path
impl PartialEq<Path> for Path
impl<'a, 'b> PartialEq<Path> for PathBuf
impl<'a, 'b> PartialEq<Path> for Cow<'a, Path>
impl<'a, 'b> PartialEq<Path> for OsStr
impl<'a, 'b> PartialEq<Path> for &'a OsStr
impl<'a, 'b> PartialEq<Path> for Cow<'a, OsStr>
impl<'a, 'b> PartialEq<Path> for OsString
impl<'a, 'b> PartialEq<PathBuf> for Path
impl<'a, 'b> PartialEq<PathBuf> for &'a Path
impl<'a, 'b> PartialOrd<&'a OsStr> for Path
impl<'a, 'b> PartialOrd<&'a Path> for PathBuf
impl<'a, 'b> PartialOrd<&'a Path> for OsStr
impl<'a, 'b> PartialOrd<&'a Path> for Cow<'b, OsStr>
impl<'a, 'b> PartialOrd<&'a Path> for OsString
impl<'a, 'b> PartialOrd<&'b Path> for Cow<'a, Path>
impl<'a, 'b> PartialOrd<Cow<'a, OsStr>> for Path
impl<'a, 'b> PartialOrd<Cow<'a, Path>> for Path
impl<'a, 'b> PartialOrd<Cow<'a, Path>> for &'b Path
impl<'a, 'b> PartialOrd<Cow<'b, OsStr>> for &'a Path
impl<'a, 'b> PartialOrd<OsStr> for Path
impl<'a, 'b> PartialOrd<OsStr> for &'a Path
impl<'a, 'b> PartialOrd<OsString> for Path
impl<'a, 'b> PartialOrd<OsString> for &'a Path
impl PartialOrd<Path> for Path
impl<'a, 'b> PartialOrd<Path> for PathBuf
impl<'a, 'b> PartialOrd<Path> for Cow<'a, Path>
impl<'a, 'b> PartialOrd<Path> for OsStr
impl<'a, 'b> PartialOrd<Path> for &'a OsStr
impl<'a, 'b> PartialOrd<Path> for Cow<'a, OsStr>
impl<'a, 'b> PartialOrd<Path> for OsString
impl<'a, 'b> PartialOrd<PathBuf> for Path
impl<'a, 'b> PartialOrd<PathBuf> for &'a Path
impl ToOwned for Path
impl AsRef<OsStr> for PathBuf
impl AsRef<Path> for PathBuf
impl Borrow<Path> for PathBuf
impl Clone for PathBuf
impl Debug for PathBuf
impl Default for PathBuf
impl Deref for PathBuf
impl<P: AsRef<Path>> Extend<P> for PathBuf
impl<T: ?Sized + AsRef<OsStr>> From<&'_ T> for PathBuf
impl<'a> From<&'a PathBuf> for Cow<'a, Path>
impl From<Box<Path, Global>> for PathBuf
impl<'a> From<Cow<'a, Path>> for PathBuf
impl From<OsString> for PathBuf
impl From<PathBuf> for Box<Path>
impl From<PathBuf> for OsString
impl<'a> From<PathBuf> for Cow<'a, Path>
impl From<PathBuf> for Arc<Path>
impl From<PathBuf> for Rc<Path>
impl From<String> for PathBuf
impl<P: AsRef<Path>> FromIterator<P> for PathBuf
impl FromStr for PathBuf
impl Hash for PathBuf
impl<'a> IntoIterator for &'a PathBuf
impl Ord for PathBuf
impl<'a, 'b> PartialEq<&'a OsStr> for PathBuf
impl<'a, 'b> PartialEq<&'a Path> for PathBuf
impl<'a, 'b> PartialEq<Cow<'a, OsStr>> for PathBuf
impl<'a, 'b> PartialEq<Cow<'a, Path>> for PathBuf
impl<'a, 'b> PartialEq<OsStr> for PathBuf
impl<'a, 'b> PartialEq<OsString> for PathBuf
impl<'a, 'b> PartialEq<Path> for PathBuf
impl PartialEq<PathBuf> for PathBuf
impl<'a, 'b> PartialEq<PathBuf> for Path
impl<'a, 'b> PartialEq<PathBuf> for &'a Path
impl<'a, 'b> PartialEq<PathBuf> for Cow<'a, Path>
impl<'a, 'b> PartialEq<PathBuf> for OsStr
impl<'a, 'b> PartialEq<PathBuf> for &'a OsStr
impl<'a, 'b> PartialEq<PathBuf> for Cow<'a, OsStr>
impl<'a, 'b> PartialEq<PathBuf> for OsString
impl<'a, 'b> PartialOrd<&'a OsStr> for PathBuf
impl<'a, 'b> PartialOrd<&'a Path> for PathBuf
impl<'a, 'b> PartialOrd<Cow<'a, OsStr>> for PathBuf
impl<'a, 'b> PartialOrd<Cow<'a, Path>> for PathBuf
impl<'a, 'b> PartialOrd<OsStr> for PathBuf
impl<'a, 'b> PartialOrd<OsString> for PathBuf
impl<'a, 'b> PartialOrd<Path> for PathBuf
impl PartialOrd<PathBuf> for PathBuf
impl<'a, 'b> PartialOrd<PathBuf> for Path
impl<'a, 'b> PartialOrd<PathBuf> for &'a Path
impl<'a, 'b> PartialOrd<PathBuf> for Cow<'a, Path>
impl<'a, 'b> PartialOrd<PathBuf> for OsStr
impl<'a, 'b> PartialOrd<PathBuf> for &'a OsStr
impl<'a, 'b> PartialOrd<PathBuf> for Cow<'a, OsStr>
impl<'a, 'b> PartialOrd<PathBuf> for OsString
impl<'a> Clone for Prefix<'a>
impl<'a> Debug for Prefix<'a>
impl<'a> Hash for Prefix<'a>
impl<'a> Ord for Prefix<'a>
impl<'a> PartialEq<Prefix<'a>> for Prefix<'a>
impl<'a> PartialOrd<Prefix<'a>> for Prefix<'a>
impl<'a> Clone for PrefixComponent<'a>
impl<'a> Debug for PrefixComponent<'a>
impl Hash for PrefixComponent<'_>
impl Ord for PrefixComponent<'_>
impl<'a> PartialEq<PrefixComponent<'a>> for PrefixComponent<'a>
impl<'a> PartialOrd<PrefixComponent<'a>> for PrefixComponent<'a>
impl Clone for StripPrefixError
impl Debug for StripPrefixError
impl Display for StripPrefixError
impl Error for StripPrefixError
impl PartialEq<StripPrefixError> for StripPrefixError
impl<P> Clone for Pin<P> where    P: Clone,
impl<P> Debug for Pin<P> where    P: Debug,
impl<P> Deref for Pin<P> where    P: Deref,
impl<P> DerefMut for Pin<P> where    P: DerefMut,    <P as Deref>::Target: Unpin,
impl<P> Display for Pin<P> where    P: Display,
impl<T, A> From<Box<T, A>> for Pin<Box<T, A>> where    A: Allocator + 'static,    T: ?Sized,
impl<P> Future for Pin<P> where    P: DerefMut,    <P as Deref>::Target: Future,
impl<'_, G, R> Generator<R> for Pin<&'_ mut G> where    G: Generator<R> + ?Sized,
impl<G, R, A> Generator<R> for Pin<Box<G, A>> where    G: Generator<R> + ?Sized,    A: Allocator + 'static,
impl<P> Hash for Pin<P> where    P: Deref,    <P as Deref>::Target: Hash,
impl<P> Ord for Pin<P> where    P: Deref,    <P as Deref>::Target: Ord,
impl<P, Q> PartialEq<Pin<Q>> for Pin<P> where    P: Deref,    Q: Deref,    <P as Deref>::Target: PartialEq<<Q as Deref>::Target>,
impl<P, Q> PartialOrd<Pin<Q>> for Pin<P> where    P: Deref,    Q: Deref,    <P as Deref>::Target: PartialOrd<<Q as Deref>::Target>,
impl<P> Pointer for Pin<P> where    P: Pointer,
impl<P> Stream for Pin<P> where    P: DerefMut,    <P as Deref>::Target: Stream,
impl AsHandle for Child
impl AsRawHandle for Child
impl ChildExt for Child
impl Debug for Child
impl From<Child> for OwnedHandle
impl IntoRawHandle for Child
impl CommandExt for Command
impl CommandExt for Command
impl CommandExt for Command
impl Debug for Command
impl<T> Clone for NonNull<T> where    T: ?Sized,
impl<T> Debug for NonNull<T> where    T: ?Sized,
impl<'_, T> From<&'_ T> for NonNull<T> where    T: ?Sized,
impl<'_, T> From<&'_ mut T> for NonNull<T> where    T: ?Sized,
impl<T> Hash for NonNull<T> where    T: ?Sized,
impl<T> Ord for NonNull<T> where    T: ?Sized,
impl<T> PartialEq<NonNull<T>> for NonNull<T> where    T: ?Sized,
impl<T> PartialOrd<NonNull<T>> for NonNull<T> where    T: ?Sized,
impl<T> Pointer for NonNull<T> where    T: ?Sized,
impl<T> AsRef<T> for Rc<T> where    T: ?Sized,
impl<T> Borrow<T> for Rc<T> where    T: ?Sized,
impl<T> Clone for Rc<T> where    T: ?Sized,
impl<T> Debug for Rc<T> where    T: Debug + ?Sized,
impl<T> Default for Rc<T> where    T: Default,
impl<T> Deref for Rc<T> where    T: ?Sized,
impl<T> Display for Rc<T> where    T: Display + ?Sized,
impl<T> Drop for Rc<T> where    T: ?Sized,
impl<'_, T> From<&'_ [T]> for Rc<[T]> where    T: Clone,
impl From<&'_ CStr> for Rc<CStr>
impl From<&'_ OsStr> for Rc<OsStr>
impl From<&'_ Path> for Rc<Path>
impl<'_> From<&'_ str> for Rc<str>
impl<T> From<Box<T, Global>> for Rc<T> where    T: ?Sized,
impl From<CString> for Rc<CStr>
impl<'a, B> From<Cow<'a, B>> for Rc<B> where    B: ToOwned + ?Sized,    Rc<B>: From<&'a B>,    Rc<B>: From<<B as ToOwned>::Owned>,
impl From<OsString> for Rc<OsStr>
impl From<PathBuf> for Rc<Path>
impl From<String> for Rc<str>
impl<T> From<T> for Rc<T>
impl<T> From<Vec<T, Global>> for Rc<[T]>
impl<T> FromIterator<T> for Rc<[T]>
impl<T> Hash for Rc<T> where    T: Hash + ?Sized,
impl<T> Ord for Rc<T> where    T: Ord + ?Sized,
impl<T> PartialEq<Rc<T>> for Rc<T> where    T: PartialEq<T> + ?Sized,
impl<T> PartialOrd<Rc<T>> for Rc<T> where    T: PartialOrd<T> + ?Sized,
impl<T> Pointer for Rc<T> where    T: ?Sized,
impl<T, const N: usize> TryFrom<Rc<[T]>> for Rc<[T; N]>
impl<T> Clone for IntoIter<T> where    T: Clone,
impl<T> Debug for IntoIter<T> where    T: Debug,
impl<T> DoubleEndedIterator for IntoIter<T>
impl<T> ExactSizeIterator for IntoIter<T>
impl<T> Iterator for IntoIter<T>
impl<'_, T> Clone for Iter<'_, T>
impl<'a, T> Debug for Iter<'a, T> where    T: 'a + Debug,
impl<'a, T> DoubleEndedIterator for Iter<'a, T>
impl<'_, T> ExactSizeIterator for Iter<'_, T>
impl<'a, T> Iterator for Iter<'a, T>
impl<'a, T> Debug for IterMut<'a, T> where    T: 'a + Debug,
impl<'a, T> DoubleEndedIterator for IterMut<'a, T>
impl<'_, T> ExactSizeIterator for IterMut<'_, T>
impl<'a, T> Iterator for IterMut<'a, T>
impl<T, E> Clone for Result<T, E> where    T: Clone,    E: Clone,
impl<T, E> Debug for Result<T, E> where    T: Debug,    E: Debug,
impl<'_> From<&'_ StreamResult> for Result<MZStatus, MZError>
impl From<StreamResult> for Result<MZStatus, MZError>
impl<A, E, V> FromIterator<Result<A, E>> for Result<V, E> where    V: FromIterator<A>,
impl<T, E, F> FromResidual<Result<Infallible, E>> for Poll<Option<Result<T, F>>> where    F: From<E>,
impl<T, E, F> FromResidual<Result<Infallible, E>> for Result<T, F> where    F: From<E>,
impl<T, E, F> FromResidual<Result<Infallible, E>> for Poll<Result<T, F>> where    F: From<E>,
impl<T, E> Hash for Result<T, E> where    T: Hash,    E: Hash,
impl<'a, T, E> IntoIterator for &'a mut Result<T, E>
impl<'a, T, E> IntoIterator for &'a Result<T, E>
impl<T, E> IntoIterator for Result<T, E>
impl<T, E> Ord for Result<T, E> where    T: Ord,    E: Ord,
impl<T, E> PartialEq<Result<T, E>> for Result<T, E> where    T: PartialEq<T>,    E: PartialEq<E>,
impl<T, E> PartialOrd<Result<T, E>> for Result<T, E> where    T: PartialOrd<T>,    E: PartialOrd<E>,
impl<T, U, E> Product<Result<U, E>> for Result<T, E> where    T: Product<U>,
impl<T, E> Residual<T> for Result<Infallible, E>
impl<T, U, E> Sum<Result<U, E>> for Result<T, E> where    T: Sum<U>,
impl<E: Debug> Termination for Result<(), E>
impl<E: Debug> Termination for Result<!, E>
impl<E: Debug> Termination for Result<Infallible, E>
impl<T, E> Try for Result<T, E>
impl<'a> Clone for Bytes<'a>
impl<'a> Debug for Bytes<'a>
impl<'_> DoubleEndedIterator for Bytes<'_>
impl<'_> ExactSizeIterator for Bytes<'_>
impl<'_> Iterator for Bytes<'_>
impl<'a> Clone for CharIndices<'a>
impl<'a> Debug for CharIndices<'a>
impl<'a> DoubleEndedIterator for CharIndices<'a>
impl<'a> Iterator for CharIndices<'a>
impl<'a> Clone for Chars<'a>
impl<'_> Debug for Chars<'_>
impl<'a> DoubleEndedIterator for Chars<'a>
impl<'a> Iterator for Chars<'a>
impl<'a> Clone for EncodeUtf16<'a>
impl<'_> Debug for EncodeUtf16<'_>
impl<'a> Iterator for EncodeUtf16<'a>
impl<'a> Clone for EscapeDebug<'a>
impl<'a> Debug for EscapeDebug<'a>
impl<'a> Display for EscapeDebug<'a>
impl<'a> Iterator for EscapeDebug<'a>
impl<'a> Clone for EscapeDefault<'a>
impl<'a> Debug for EscapeDefault<'a>
impl<'a> Display for EscapeDefault<'a>
impl<'a> Iterator for EscapeDefault<'a>
impl<'a> Clone for EscapeUnicode<'a>
impl<'a> Debug for EscapeUnicode<'a>
impl<'a> Display for EscapeUnicode<'a>
impl<'a> Iterator for EscapeUnicode<'a>
impl<'a> Clone for Lines<'a>
impl<'a> Debug for Lines<'a>
impl<'a> DoubleEndedIterator for Lines<'a>
impl<'a> Iterator for Lines<'a>
impl<'a, P> Clone for MatchIndices<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: Clone,
impl<'a, P> Debug for MatchIndices<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: Debug,
impl<'a, P> DoubleEndedIterator for MatchIndices<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: DoubleEndedSearcher<'a>,
impl<'a, P> Iterator for MatchIndices<'a, P> where    P: Pattern<'a>,
impl<'a, P> Clone for Matches<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: Clone,
impl<'a, P> Debug for Matches<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: Debug,
impl<'a, P> DoubleEndedIterator for Matches<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: DoubleEndedSearcher<'a>,
impl<'a, P> Iterator for Matches<'a, P> where    P: Pattern<'a>,
impl Clone for ParseBoolError
impl Debug for ParseBoolError
impl Display for ParseBoolError
impl Error for ParseBoolError
impl PartialEq<ParseBoolError> for ParseBoolError
impl<'a, P> Clone for RMatchIndices<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: Clone,
impl<'a, P> Debug for RMatchIndices<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: Debug,
impl<'a, P> DoubleEndedIterator for RMatchIndices<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: DoubleEndedSearcher<'a>,
impl<'a, P> Iterator for RMatchIndices<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: ReverseSearcher<'a>,
impl<'a, P> Clone for RMatches<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: Clone,
impl<'a, P> Debug for RMatches<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: Debug,
impl<'a, P> DoubleEndedIterator for RMatches<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: DoubleEndedSearcher<'a>,
impl<'a, P> Iterator for RMatches<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: ReverseSearcher<'a>,
impl<'a, P> Clone for RSplit<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: Clone,
impl<'a, P> Debug for RSplit<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: Debug,
impl<'a, P> DoubleEndedIterator for RSplit<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: DoubleEndedSearcher<'a>,
impl<'a, P> Iterator for RSplit<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: ReverseSearcher<'a>,
impl<'a, P> Clone for RSplitN<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: Clone,
impl<'a, P> Debug for RSplitN<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: Debug,
impl<'a, P> Iterator for RSplitN<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: ReverseSearcher<'a>,
impl<'a, P> Clone for RSplitTerminator<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: Clone,
impl<'a, P> Debug for RSplitTerminator<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: Debug,
impl<'a, P> DoubleEndedIterator for RSplitTerminator<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: DoubleEndedSearcher<'a>,
impl<'a, P> Iterator for RSplitTerminator<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: ReverseSearcher<'a>,
impl<'a, P> Clone for Split<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: Clone,
impl<'a, P> Debug for Split<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: Debug,
impl<'a, P> DoubleEndedIterator for Split<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: DoubleEndedSearcher<'a>,
impl<'a, P> Iterator for Split<'a, P> where    P: Pattern<'a>,
impl<'a> Clone for SplitAsciiWhitespace<'a>
impl<'a> Debug for SplitAsciiWhitespace<'a>
impl<'a> DoubleEndedIterator for SplitAsciiWhitespace<'a>
impl<'a> Iterator for SplitAsciiWhitespace<'a>
impl<'a, P> Clone for SplitInclusive<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: Clone,
impl<'a, P> Debug for SplitInclusive<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: Debug,
impl<'a, P> DoubleEndedIterator for SplitInclusive<'a, P> where    P: Pattern<'a>,    <P as Pattern<'a>>::Searcher: ReverseSearcher<'a>,
impl<'a, P> Iterator for SplitInclusive<'a, P> where    P: Pattern<'a>,
impl<'a> Clone for SplitWhitespace<'a>
impl<'a> Debug for SplitWhitespace<'a>
impl<'a> DoubleEndedIterator for SplitWhitespace<'a>
impl<'a> Iterator for SplitWhitespace<'a>
impl Clone for Utf8Error
impl Debug for Utf8Error
impl Display for Utf8Error
impl Error for Utf8Error
impl PartialEq<Utf8Error> for Utf8Error
impl<'_> Add<&'_ str> for String
impl<'_> AddAssign<&'_ str> for String
impl AsMut<str> for String
impl AsRef<[u8]> for String
impl AsRef<OsStr> for String
impl AsRef<Path> for String
impl AsRef<str> for String
impl Borrow<str> for String
impl BorrowMut<str> for String
impl Clone for String
impl Debug for String
impl Default for String
impl Deref for String
impl DerefMut for String
impl Display for String
impl<'a> Extend<&'a char> for String
impl<'a> Extend<&'a str> for String
impl Extend<Box<str, Global>> for String
impl<'a> Extend<Cow<'a, str>> for String
impl Extend<String> for String
impl Extend<char> for String
impl<'_> From<&'_ String> for String
impl<'_> From<&'_ mut str> for String
impl<'_> From<&'_ str> for String
impl<'a> From<&'a String> for Cow<'a, str>
impl From<Box<str, Global>> for String
impl<'a> From<Cow<'a, str>> for String
impl From<String> for Vec<u8, Global>
impl From<String> for Arc<str>
impl From<String> for Box<str, Global>
impl<'a> From<String> for Cow<'a, str>
impl From<String> for Rc<str>
impl From<String> for Box<dyn Error + Send + Sync>
impl From<String> for Box<dyn Error>
impl From<String> for OsString
impl From<String> for PathBuf
impl From<char> for String
impl<'a> FromIterator<&'a char> for String
impl<'a> FromIterator<&'a str> for String
impl FromIterator<Box<str, Global>> for String
impl<'a> FromIterator<Cow<'a, str>> for String
impl<'a> FromIterator<String> for Cow<'a, str>
impl FromIterator<String> for String
impl FromIterator<char> for String
impl FromStr for String
impl Hash for String
impl Index<Range<usize>> for String
impl Index<RangeFrom<usize>> for String
impl Index<RangeFull> for String
impl Index<RangeInclusive<usize>> for String
impl Index<RangeTo<usize>> for String
impl Index<RangeToInclusive<usize>> for String
impl IndexMut<Range<usize>> for String
impl IndexMut<RangeFrom<usize>> for String
impl IndexMut<RangeFull> for String
impl IndexMut<RangeInclusive<usize>> for String
impl IndexMut<RangeTo<usize>> for String
impl IndexMut<RangeToInclusive<usize>> for String
impl Ord for String
impl<'a, 'b> PartialEq<&'a str> for String
impl<'a, 'b> PartialEq<Cow<'a, str>> for String
impl<'a, 'b> PartialEq<String> for Cow<'a, str>
impl<'a, 'b> PartialEq<String> for str
impl PartialEq<String> for String
impl<'a, 'b> PartialEq<String> for &'a str
impl<'a, 'b> PartialEq<str> for String
impl PartialOrd<String> for String
impl<'a, 'b> Pattern<'a> for &'b String
impl ToSocketAddrs for String
impl ToString for String
impl Write for String
impl<T> AsRef<T> for Arc<T> where    T: ?Sized,
impl<T> Borrow<T> for Arc<T> where    T: ?Sized,
impl<T> Clone for Arc<T> where    T: ?Sized,
impl<T> Debug for Arc<T> where    T: Debug + ?Sized,
impl<T> Default for Arc<T> where    T: Default,
impl<T> Deref for Arc<T> where    T: ?Sized,
impl<T> Display for Arc<T> where    T: Display + ?Sized,
impl<T> Drop for Arc<T> where    T: ?Sized,
impl<T: Error + ?Sized> Error for Arc<T>
impl<'_, T> From<&'_ [T]> for Arc<[T]> where    T: Clone,
impl From<&'_ CStr> for Arc<CStr>
impl From<&'_ OsStr> for Arc<OsStr>
impl From<&'_ Path> for Arc<Path>
impl<'_> From<&'_ str> for Arc<str>
impl<W> From<Arc<W>> for Waker where    W: 'static + Wake + Send + Sync,
impl<W> From<Arc<W>> for RawWaker where    W: 'static + Wake + Send + Sync,
impl<T> From<Box<T, Global>> for Arc<T> where    T: ?Sized,
impl From<CString> for Arc<CStr>
impl<'a, B> From<Cow<'a, B>> for Arc<B> where    B: ToOwned + ?Sized,    Arc<B>: From<&'a B>,    Arc<B>: From<<B as ToOwned>::Owned>,
impl From<OsString> for Arc<OsStr>
impl From<PathBuf> for Arc<Path>
impl From<String> for Arc<str>
impl<T> From<T> for Arc<T>
impl<T> From<Vec<T, Global>> for Arc<[T]>
impl<T> FromIterator<T> for Arc<[T]>
impl<T> Hash for Arc<T> where    T: Hash + ?Sized,
impl<T> Ord for Arc<T> where    T: Ord + ?Sized,
impl<T> PartialEq<Arc<T>> for Arc<T> where    T: PartialEq<T> + ?Sized,
impl<T> PartialOrd<Arc<T>> for Arc<T> where    T: PartialOrd<T> + ?Sized,
impl<T> Pointer for Arc<T> where    T: ?Sized,
impl<T, const N: usize> TryFrom<Arc<[T]>> for Arc<[T; N]>
impl Debug for Barrier
impl Debug for Condvar
impl Default for Condvar
impl<T: ?Sized + Debug> Debug for Mutex<T>
impl<T: ?Sized + Default> Default for Mutex<T>
impl<T> From<T> for Mutex<T>
impl Debug for Once
impl<T: ?Sized + Debug> Debug for RwLock<T>
impl<T: Default> Default for RwLock<T>
impl<T> From<T> for RwLock<T>
impl<T: Debug> Debug for RwLockReadGuard<'_, T>
impl<T: ?Sized> Deref for RwLockReadGuard<'_, T>
impl<T: ?Sized + Display> Display for RwLockReadGuard<'_, T>
impl<T: ?Sized> Drop for RwLockReadGuard<'_, T>
impl<T: Debug> Debug for RwLockWriteGuard<'_, T>
impl<T: ?Sized> Deref for RwLockWriteGuard<'_, T>
impl<T: ?Sized> DerefMut for RwLockWriteGuard<'_, T>
impl<T: ?Sized + Display> Display for RwLockWriteGuard<'_, T>
impl<T: ?Sized> Drop for RwLockWriteGuard<'_, T>
impl<T> Clone for Weak<T> where    T: ?Sized,
impl<T> Debug for Weak<T> where    T: Debug + ?Sized,
impl<T> Default for Weak<T>
impl<T> Drop for Weak<T> where    T: ?Sized,
impl<'_> Debug for Context<'_>
impl<T> Clone for Poll<T> where    T: Clone,
impl<T> Debug for Poll<T> where    T: Debug,
impl<T> From<T> for Poll<T>
impl<T> FromResidual<Ready<Infallible>> for Poll<T>
impl<T, E, F> FromResidual<Result<Infallible, E>> for Poll<Option<Result<T, F>>> where    F: From<E>,
impl<T, E, F> FromResidual<Result<Infallible, E>> for Poll<Result<T, F>> where    F: From<E>,
impl<T> Hash for Poll<T> where    T: Hash,
impl<T> Ord for Poll<T> where    T: Ord,
impl<T> PartialEq<Poll<T>> for Poll<T> where    T: PartialEq<T>,
impl<T> PartialOrd<Poll<T>> for Poll<T> where    T: PartialOrd<T>,
impl<T, E> Try for Poll<Option<Result<T, E>>>
impl<T, E> Try for Poll<Result<T, E>>
impl Debug for Builder
impl<T> AsHandle for JoinHandle<T>
impl<T> AsRawHandle for JoinHandle<T>
impl<T> Debug for JoinHandle<T>
impl<T> From<JoinHandle<T>> for OwnedHandle
impl<T> IntoRawHandle for JoinHandle<T>
impl<T> JoinHandleExt for JoinHandle<T>
impl<T: 'static> Debug for LocalKey<T>
impl Clone for Thread
impl Debug for Thread
impl Clone for ThreadId
impl Debug for ThreadId
impl Hash for ThreadId
impl PartialEq<ThreadId> for ThreadId
impl Add<Duration> for Duration
impl Add<Duration> for Instant
impl Add<Duration> for SystemTime
impl AddAssign<Duration> for Duration
impl AddAssign<Duration> for Instant
impl AddAssign<Duration> for SystemTime
impl Clone for Duration
impl Debug for Duration
impl Default for Duration
impl Div<u32> for Duration
impl DivAssign<u32> for Duration
impl Hash for Duration
impl Mul<Duration> for u32
impl Mul<u32> for Duration
impl MulAssign<u32> for Duration
impl Ord for Duration
impl PartialEq<Duration> for Duration
impl PartialOrd<Duration> for Duration
impl Sub<Duration> for Duration
impl Sub<Duration> for Instant
impl Sub<Duration> for SystemTime
impl SubAssign<Duration> for Duration
impl SubAssign<Duration> for Instant
impl SubAssign<Duration> for SystemTime
impl<'a> Sum<&'a Duration> for Duration
impl Sum<Duration> for Duration
impl Add<Duration> for Instant
impl AddAssign<Duration> for Instant
impl Clone for Instant
impl Debug for Instant
impl Hash for Instant
impl Ord for Instant
impl PartialEq<Instant> for Instant
impl PartialOrd<Instant> for Instant
impl Sub<Duration> for Instant
impl Sub<Instant> for Instant
impl SubAssign<Duration> for Instant
impl Add<Duration> for SystemTime
impl AddAssign<Duration> for SystemTime
impl Clone for SystemTime
impl Debug for SystemTime
impl Hash for SystemTime
impl Ord for SystemTime
impl PartialEq<SystemTime> for SystemTime
impl PartialOrd<SystemTime> for SystemTime
impl Sub<Duration> for SystemTime
impl SubAssign<Duration> for SystemTime
impl<'a, I, A> Debug for Splice<'a, I, A> where    I: 'a + Debug + Iterator,    A: 'a + Debug + Allocator,    <I as Iterator>::Item: Debug,
impl<'_, I, A> DoubleEndedIterator for Splice<'_, I, A> where    I: Iterator,    A: Allocator,
impl<'_, I, A> Drop for Splice<'_, I, A> where    I: Iterator,    A: Allocator,
impl<'_, I, A> ExactSizeIterator for Splice<'_, I, A> where    I: Iterator,    A: Allocator,
impl<'_, I, A> Iterator for Splice<'_, I, A> where    I: Iterator,    A: Allocator,
impl<T, A> AsMut<[T]> for Vec<T, A> where    A: Allocator,
impl<T, A> AsMut<Vec<T, A>> for Vec<T, A> where    A: Allocator,
impl<T, A> AsRef<[T]> for Vec<T, A> where    A: Allocator,
impl<T, A> AsRef<Vec<T, A>> for Vec<T, A> where    A: Allocator,
impl<T> Borrow<[T]> for Vec<T, Global>
impl<T> BorrowMut<[T]> for Vec<T, Global>
impl<T, A> Clone for Vec<T, A> where    T: Clone,    A: Allocator + Clone,
impl<T, A> Debug for Vec<T, A> where    T: Debug,    A: Allocator,
impl<T> Default for Vec<T, Global>
impl<T, A> Deref for Vec<T, A> where    A: Allocator,
impl<T, A> DerefMut for Vec<T, A> where    A: Allocator,
impl<T, A> Drop for Vec<T, A> where    A: Allocator,
impl<'a, T, A> Extend<&'a T> for Vec<T, A> where    T: 'a + Copy,    A: 'a + Allocator,
impl<T, A> Extend<T> for Vec<T, A> where    A: Allocator,
impl<'_, T> From<&'_ [T]> for Vec<T, Global> where    T: Clone,
impl<'_, T> From<&'_ mut [T]> for Vec<T, Global> where    T: Clone,
impl<'_> From<&'_ str> for Vec<u8, Global>
impl<'a, T> From<&'a Vec<T, Global>> for Cow<'a, [T]> where    T: Clone,
impl<T, const N: usize> From<[T; N]> for Vec<T, Global>
impl<T> From<BinaryHeap<T>> for Vec<T, Global>
impl<T, A> From<Box<[T], A>> for Vec<T, A> where    A: Allocator,
impl From<CString> for Vec<u8>
impl<'a, T> From<Cow<'a, [T]>> for Vec<T, Global> where    [T]: ToOwned,    <[T] as ToOwned>::Owned == Vec<T, Global>,
impl From<String> for Vec<u8, Global>
impl From<Vec<NonZeroU8, Global>> for CString
impl<T, A> From<Vec<T, A>> for Box<[T], A> where    A: Allocator,
impl<T, A> From<Vec<T, A>> for VecDeque<T, A> where    A: Allocator,
impl<T> From<Vec<T, Global>> for Rc<[T]>
impl<T> From<Vec<T, Global>> for Arc<[T]>
impl<'a, T> From<Vec<T, Global>> for Cow<'a, [T]> where    T: Clone,
impl<T> From<Vec<T, Global>> for BinaryHeap<T> where    T: Ord,
impl<T, A> From<VecDeque<T, A>> for Vec<T, A> where    A: Allocator,
impl<T> FromIterator<T> for Vec<T, Global>
impl<T, A> Hash for Vec<T, A> where    T: Hash,    A: Allocator,
impl<T, I, A> Index<I> for Vec<T, A> where    I: SliceIndex<[T]>,    A: Allocator,
impl<T, I, A> IndexMut<I> for Vec<T, A> where    I: SliceIndex<[T]>,    A: Allocator,
impl<'a, T, A> IntoIterator for &'a Vec<T, A> where    A: Allocator,
impl<T, A> IntoIterator for Vec<T, A> where    A: Allocator,
impl<'a, T, A> IntoIterator for &'a mut Vec<T, A> where    A: Allocator,
impl<T, A> Ord for Vec<T, A> where    T: Ord,    A: Allocator,
impl<'_, T, U, A, const N: usize> PartialEq<&'_ [U; N]> for Vec<T, A> where    A: Allocator,    T: PartialEq<U>,
impl<'_, T, U, A> PartialEq<&'_ [U]> for Vec<T, A> where    A: Allocator,    T: PartialEq<U>,
impl<'_, T, U, A> PartialEq<&'_ mut [U]> for Vec<T, A> where    A: Allocator,    T: PartialEq<U>,
impl<T, U, A, const N: usize> PartialEq<[U; N]> for Vec<T, A> where    A: Allocator,    T: PartialEq<U>,
impl<T, U, A> PartialEq<[U]> for Vec<T, A> where    A: Allocator,    T: PartialEq<U>,
impl<'_, T, U, A> PartialEq<Vec<U, A>> for Cow<'_, [T]> where    A: Allocator,    T: PartialEq<U> + Clone,
impl<'_, T, U, A> PartialEq<Vec<U, A>> for &'_ [T] where    A: Allocator,    T: PartialEq<U>,
impl<T, U, A> PartialEq<Vec<U, A>> for Vec<T, A> where    A: Allocator,    T: PartialEq<U>,
impl<T, U, A> PartialEq<Vec<U, A>> for [T] where    A: Allocator,    T: PartialEq<U>,
impl<'_, T, U, A> PartialEq<Vec<U, A>> for &'_ mut [T] where    A: Allocator,    T: PartialEq<U>,
impl<T, U, A> PartialEq<Vec<U, A>> for VecDeque<T, A> where    A: Allocator,    T: PartialEq<U>,
impl<T, A> PartialOrd<Vec<T, A>> for Vec<T, A> where    T: PartialOrd<T>,    A: Allocator,
impl<T, A, const N: usize> TryFrom<Vec<T, A>> for [T; N] where    A: Allocator,
impl<A: Allocator> Write for Vec<u8, A>
impl<'_> Add<&'_ str> for String
impl<'a> Add<&'a str> for Cow<'a, str>
impl<'_> AddAssign<&'_ str> for String
impl<'a> AddAssign<&'a str> for Cow<'a, str>
impl AsMut<str> for str
impl AsMut<str> for String
impl AsRef<[u8]> for str
impl AsRef<OsStr> for str
impl AsRef<Path> for str
impl AsRef<str> for str
impl AsRef<str> for String
impl<'a> AsRef<str> for Drain<'a>
impl AsciiExt for str
impl Borrow<str> for String
impl BorrowMut<str> for String
impl<S> Concat<str> for [S] where    S: Borrow<str>,
impl Debug for str
impl<'_> Default for &'_ str
impl<'_> Default for &'_ mut str
impl Display for str
impl<'a> Extend<&'a str> for String
impl<'_> From<&'_ mut str> for String
impl<'_> From<&'_ str> for Box<str, Global>
impl<'_> From<&'_ str> for Vec<u8, Global>
impl<'_> From<&'_ str> for String
impl<'_> From<&'_ str> for Rc<str>
impl<'_> From<&'_ str> for Arc<str>
impl<'a> From<&'_ str> for Box<dyn Error + Send + Sync + 'a>
impl From<&'_ str> for Box<dyn Error>
impl<'a> From<&'a str> for Cow<'a, str>
impl<'a> FromIterator<&'a str> for String
impl<'a, 'b> FromIterator<&'b str> for Cow<'a, str>
impl Hash for str
impl<I> Index<I> for str where    I: SliceIndex<str>,
impl<I> IndexMut<I> for str where    I: SliceIndex<str>,
impl<'_, S> Join<&'_ str> for [S] where    S: Borrow<str>,
impl Ord for str
impl PartialEq<&'_ str> for OsString
impl<'a, 'b> PartialEq<&'a str> for String
impl<'a, 'b> PartialEq<&'b str> for Cow<'a, str>
impl<'a, 'b> PartialEq<Cow<'a, str>> for &'b str
impl<'a, 'b> PartialEq<Cow<'a, str>> for str
impl PartialEq<OsStr> for str
impl PartialEq<OsString> for str
impl<'a> PartialEq<OsString> for &'a str
impl<'a, 'b> PartialEq<String> for str
impl<'a, 'b> PartialEq<String> for &'a str
impl PartialEq<str> for str
impl<'a, 'b> PartialEq<str> for Cow<'a, str>
impl<'a, 'b> PartialEq<str> for String
impl PartialEq<str> for OsString
impl PartialEq<str> for OsStr
impl PartialOrd<str> for str
impl PartialOrd<str> for OsString
impl PartialOrd<str> for OsStr
impl<'a, 'b> Pattern<'a> for &'b str
impl SliceIndex<str> for Range<usize>
impl SliceIndex<str> for RangeTo<usize>
impl SliceIndex<str> for RangeFull
impl SliceIndex<str> for RangeFrom<usize>
impl SliceIndex<str> for RangeToInclusive<usize>
impl SliceIndex<str> for RangeInclusive<usize>
impl ToOwned for str
impl ToSocketAddrs for str
impl ToString for str
impl<'_> Add<&'_ u32> for u32
impl<'_, '_> Add<&'_ u32> for &'_ u32
impl<'a> Add<u32> for &'a u32
impl Add<u32> for u32
impl<'_> AddAssign<&'_ u32> for u32
impl AddAssign<u32> for u32
impl Binary for u32
impl<'_> BitAnd<&'_ u32> for u32
impl<'_, '_> BitAnd<&'_ u32> for &'_ u32
impl<'a> BitAnd<u32> for &'a u32
impl BitAnd<u32> for u32
impl<'_> BitAndAssign<&'_ u32> for u32
impl BitAndAssign<u32> for u32
impl<'_> BitOr<&'_ u32> for u32
impl<'_, '_> BitOr<&'_ u32> for &'_ u32
impl BitOr<NonZeroU32> for u32
impl BitOr<u32> for u32
impl<'a> BitOr<u32> for &'a u32
impl BitOr<u32> for NonZeroU32
impl<'_> BitOrAssign<&'_ u32> for u32
impl BitOrAssign<u32> for NonZeroU32
impl BitOrAssign<u32> for u32
impl<'_> BitXor<&'_ u32> for u32
impl<'_, '_> BitXor<&'_ u32> for &'_ u32
impl BitXor<u32> for u32
impl<'a> BitXor<u32> for &'a u32
impl<'_> BitXorAssign<&'_ u32> for u32
impl BitXorAssign<u32> for u32
impl Clone for u32
impl Debug for u32
impl Default for u32
impl Display for u32
impl<'_> Div<&'_ u32> for u32
impl<'_, '_> Div<&'_ u32> for &'_ u32
impl Div<NonZeroU32> for u32
impl Div<u32> for Duration
impl<'a> Div<u32> for &'a u32
impl Div<u32> for u32
impl<'_> DivAssign<&'_ u32> for u32
impl DivAssign<u32> for Duration
impl DivAssign<u32> for u32
impl From<Ipv4Addr> for u32
impl From<NonZeroU32> for u32
impl From<bool> for u32
impl From<char> for u32
impl From<u16> for u32
impl From<u32> for u128
impl From<u32> for u64
impl From<u32> for i64
impl From<u32> for AtomicU32
impl From<u32> for f64
impl From<u32> for i128
impl From<u32> for Ipv4Addr
impl From<u8> for u32
impl FromStr for u32
impl Hash for u32
impl LowerExp for u32
impl LowerHex for u32
impl<'_> Mul<&'_ u32> for u32
impl<'_, '_> Mul<&'_ u32> for &'_ u32
impl Mul<Duration> for u32
impl Mul<u32> for Duration
impl Mul<u32> for u32
impl<'a> Mul<u32> for &'a u32
impl<'_> MulAssign<&'_ u32> for u32
impl MulAssign<u32> for Duration
impl MulAssign<u32> for u32
impl Not for u32
impl<'_> Not for &'_ u32
impl Octal for u32
impl Ord for u32
impl PartialEq<u32> for u32
impl PartialOrd<u32> for u32
impl<'a> Product<&'a u32> for u32
impl Product<u32> for u32
impl<'_, '_> Rem<&'_ u32> for &'_ u32
impl<'_> Rem<&'_ u32> for u32
impl Rem<NonZeroU32> for u32
impl Rem<u32> for u32
impl<'a> Rem<u32> for &'a u32
impl<'_> RemAssign<&'_ u32> for u32
impl RemAssign<u32> for u32
impl<'_, '_> Shl<&'_ i128> for &'_ u32
impl<'_> Shl<&'_ i128> for u32
impl<'_> Shl<&'_ i16> for u32
impl<'_, '_> Shl<&'_ i16> for &'_ u32
impl<'_> Shl<&'_ i32> for u32
impl<'_, '_> Shl<&'_ i32> for &'_ u32
impl<'_, '_> Shl<&'_ i64> for &'_ u32
impl<'_> Shl<&'_ i64> for u32
impl<'_> Shl<&'_ i8> for u32
impl<'_, '_> Shl<&'_ i8> for &'_ u32
impl<'_> Shl<&'_ isize> for u32
impl<'_, '_> Shl<&'_ isize> for &'_ u32
impl<'_, '_> Shl<&'_ u128> for &'_ u32
impl<'_> Shl<&'_ u128> for u32
impl<'_> Shl<&'_ u16> for u32
impl<'_, '_> Shl<&'_ u16> for &'_ u32
impl<'_, '_> Shl<&'_ u32> for &'_ u32
impl<'_, '_> Shl<&'_ u32> for &'_ i16
impl<'_, '_> Shl<&'_ u32> for &'_ i8
impl<'_, '_> Shl<&'_ u32> for &'_ u16
impl<'_, '_> Shl<&'_ u32> for &'_ u128
impl<'_, '_> Shl<&'_ u32> for &'_ isize
impl<'_> Shl<&'_ u32> for u64
impl<'_> Shl<&'_ u32> for i64
impl<'_> Shl<&'_ u32> for usize
impl<'_> Shl<&'_ u32> for u16
impl<'_> Shl<&'_ u32> for u128
impl<'_> Shl<&'_ u32> for i128
impl<'_> Shl<&'_ u32> for isize
impl<'_, '_> Shl<&'_ u32> for &'_ u64
impl<'_> Shl<&'_ u32> for i16
impl<'_, '_> Shl<&'_ u32> for &'_ i64
impl<'_, '_> Shl<&'_ u32> for &'_ i128
impl<'_> Shl<&'_ u32> for u32
impl<'_> Shl<&'_ u32> for i32
impl<'_> Shl<&'_ u32> for u8
impl<'_, '_> Shl<&'_ u32> for &'_ u8
impl<'_> Shl<&'_ u32> for i8
impl<'_, '_> Shl<&'_ u32> for &'_ i32
impl<'_, '_> Shl<&'_ u32> for &'_ usize
impl<'_, '_> Shl<&'_ u64> for &'_ u32
impl<'_> Shl<&'_ u64> for u32
impl<'_> Shl<&'_ u8> for u32
impl<'_, '_> Shl<&'_ u8> for &'_ u32
impl<'_, '_> Shl<&'_ usize> for &'_ u32
impl<'_> Shl<&'_ usize> for u32
impl Shl<i128> for u32
impl<'a> Shl<i128> for &'a u32
impl<'a> Shl<i16> for &'a u32
impl Shl<i16> for u32
impl Shl<i32> for u32
impl<'a> Shl<i32> for &'a u32
impl Shl<i64> for u32
impl<'a> Shl<i64> for &'a u32
impl<'a> Shl<i8> for &'a u32
impl Shl<i8> for u32
impl Shl<isize> for u32
impl<'a> Shl<isize> for &'a u32
impl Shl<u128> for u32
impl<'a> Shl<u128> for &'a u32
impl<'a> Shl<u16> for &'a u32
impl Shl<u16> for u32
impl Shl<u32> for usize
impl Shl<u32> for u16
impl Shl<u32> for i8
impl<'a> Shl<u32> for &'a usize
impl Shl<u32> for isize
impl<'a> Shl<u32> for &'a isize
impl Shl<u32> for i64
impl Shl<u32> for i32
impl Shl<u32> for u32
impl Shl<u32> for i128
impl<'a> Shl<u32> for &'a u64
impl<'a> Shl<u32> for &'a i16
impl Shl<u32> for u128
impl Shl<u32> for u64
impl<'a> Shl<u32> for &'a u16
impl<'a> Shl<u32> for &'a u32
impl<'a> Shl<u32> for &'a u8
impl<'a> Shl<u32> for &'a i32
impl<'a> Shl<u32> for &'a u128
impl<'a> Shl<u32> for &'a i128
impl<'a> Shl<u32> for &'a i64
impl<'a> Shl<u32> for &'a i8
impl Shl<u32> for u8
impl Shl<u32> for i16
impl Shl<u64> for u32
impl<'a> Shl<u64> for &'a u32
impl<'a> Shl<u8> for &'a u32
impl Shl<u8> for u32
impl Shl<usize> for u32
impl<'a> Shl<usize> for &'a u32
impl<'_> ShlAssign<&'_ i128> for u32
impl<'_> ShlAssign<&'_ i16> for u32
impl<'_> ShlAssign<&'_ i32> for u32
impl<'_> ShlAssign<&'_ i64> for u32
impl<'_> ShlAssign<&'_ i8> for u32
impl<'_> ShlAssign<&'_ isize> for u32
impl<'_> ShlAssign<&'_ u128> for u32
impl<'_> ShlAssign<&'_ u16> for u32
impl<'_> ShlAssign<&'_ u32> for isize
impl<'_> ShlAssign<&'_ u32> for i128
impl<'_> ShlAssign<&'_ u32> for i64
impl<'_> ShlAssign<&'_ u32> for i16
impl<'_> ShlAssign<&'_ u32> for u64
impl<'_> ShlAssign<&'_ u32> for u128
impl<'_> ShlAssign<&'_ u32> for u8
impl<'_> ShlAssign<&'_ u32> for u16
impl<'_> ShlAssign<&'_ u32> for i8
impl<'_> ShlAssign<&'_ u32> for u32
impl<'_> ShlAssign<&'_ u32> for usize
impl<'_> ShlAssign<&'_ u32> for i32
impl<'_> ShlAssign<&'_ u64> for u32
impl<'_> ShlAssign<&'_ u8> for u32
impl<'_> ShlAssign<&'_ usize> for u32
impl ShlAssign<i128> for u32
impl ShlAssign<i16> for u32
impl ShlAssign<i32> for u32
impl ShlAssign<i64> for u32
impl ShlAssign<i8> for u32
impl ShlAssign<isize> for u32
impl ShlAssign<u128> for u32
impl ShlAssign<u16> for u32
impl ShlAssign<u32> for i32
impl ShlAssign<u32> for u64
impl ShlAssign<u32> for i8
impl ShlAssign<u32> for u16
impl ShlAssign<u32> for usize
impl ShlAssign<u32> for i64
impl ShlAssign<u32> for u32
impl ShlAssign<u32> for u128
impl ShlAssign<u32> for i128
impl ShlAssign<u32> for u8
impl ShlAssign<u32> for isize
impl ShlAssign<u32> for i16
impl ShlAssign<u64> for u32
impl ShlAssign<u8> for u32
impl ShlAssign<usize> for u32
impl<'_> Shr<&'_ i128> for u32
impl<'_, '_> Shr<&'_ i128> for &'_ u32
impl<'_, '_> Shr<&'_ i16> for &'_ u32
impl<'_> Shr<&'_ i16> for u32
impl<'_> Shr<&'_ i32> for u32
impl<'_, '_> Shr<&'_ i32> for &'_ u32
impl<'_, '_> Shr<&'_ i64> for &'_ u32
impl<'_> Shr<&'_ i64> for u32
impl<'_, '_> Shr<&'_ i8> for &'_ u32
impl<'_> Shr<&'_ i8> for u32
impl<'_> Shr<&'_ isize> for u32
impl<'_, '_> Shr<&'_ isize> for &'_ u32
impl<'_, '_> Shr<&'_ u128> for &'_ u32
impl<'_> Shr<&'_ u128> for u32
impl<'_> Shr<&'_ u16> for u32
impl<'_, '_> Shr<&'_ u16> for &'_ u32
impl<'_> Shr<&'_ u32> for u64
impl<'_> Shr<&'_ u32> for u32
impl<'_> Shr<&'_ u32> for i128
impl<'_, '_> Shr<&'_ u32> for &'_ i16
impl<'_, '_> Shr<&'_ u32> for &'_ usize
impl<'_> Shr<&'_ u32> for i16
impl<'_, '_> Shr<&'_ u32> for &'_ u8
impl<'_> Shr<&'_ u32> for isize
impl<'_, '_> Shr<&'_ u32> for &'_ isize
impl<'_> Shr<&'_ u32> for i32
impl<'_> Shr<&'_ u32> for u8
impl<'_> Shr<&'_ u32> for u16
impl<'_, '_> Shr<&'_ u32> for &'_ i128
impl<'_> Shr<&'_ u32> for usize
impl<'_> Shr<&'_ u32> for i64
impl<'_, '_> Shr<&'_ u32> for &'_ i32
impl<'_, '_> Shr<&'_ u32> for &'_ u16
impl<'_, '_> Shr<&'_ u32> for &'_ u64
impl<'_, '_> Shr<&'_ u32> for &'_ i64
impl<'_> Shr<&'_ u32> for i8
impl<'_> Shr<&'_ u32> for u128
impl<'_, '_> Shr<&'_ u32> for &'_ u128
impl<'_, '_> Shr<&'_ u32> for &'_ i8
impl<'_, '_> Shr<&'_ u32> for &'_ u32
impl<'_, '_> Shr<&'_ u64> for &'_ u32
impl<'_> Shr<&'_ u64> for u32
impl<'_> Shr<&'_ u8> for u32
impl<'_, '_> Shr<&'_ u8> for &'_ u32
impl<'_, '_> Shr<&'_ usize> for &'_ u32
impl<'_> Shr<&'_ usize> for u32
impl Shr<i128> for u32
impl<'a> Shr<i128> for &'a u32
impl<'a> Shr<i16> for &'a u32
impl Shr<i16> for u32
impl<'a> Shr<i32> for &'a u32
impl Shr<i32> for u32
impl Shr<i64> for u32
impl<'a> Shr<i64> for &'a u32
impl Shr<i8> for u32
impl<'a> Shr<i8> for &'a u32
impl Shr<isize> for u32
impl<'a> Shr<isize> for &'a u32
impl Shr<u128> for u32
impl<'a> Shr<u128> for &'a u32
impl<'a> Shr<u16> for &'a u32
impl Shr<u16> for u32
impl Shr<u32> for usize
impl Shr<u32> for u32
impl Shr<u32> for isize
impl<'a> Shr<u32> for &'a i8
impl<'a> Shr<u32> for &'a i128
impl Shr<u32> for i8
impl<'a> Shr<u32> for &'a isize
impl<'a> Shr<u32> for &'a u64
impl<'a> Shr<u32> for &'a i32
impl<'a> Shr<u32> for &'a i64
impl Shr<u32> for u8
impl<'a> Shr<u32> for &'a u8
impl Shr<u32> for u64
impl Shr<u32> for u128
impl Shr<u32> for i32
impl Shr<u32> for i128
impl Shr<u32> for i64
impl<'a> Shr<u32> for &'a u128
impl<'a> Shr<u32> for &'a usize
impl Shr<u32> for u16
impl<'a> Shr<u32> for &'a i16
impl Shr<u32> for i16
impl<'a> Shr<u32> for &'a u16
impl<'a> Shr<u32> for &'a u32
impl Shr<u64> for u32
impl<'a> Shr<u64> for &'a u32
impl<'a> Shr<u8> for &'a u32
impl Shr<u8> for u32
impl Shr<usize> for u32
impl<'a> Shr<usize> for &'a u32
impl<'_> ShrAssign<&'_ i128> for u32
impl<'_> ShrAssign<&'_ i16> for u32
impl<'_> ShrAssign<&'_ i32> for u32
impl<'_> ShrAssign<&'_ i64> for u32
impl<'_> ShrAssign<&'_ i8> for u32
impl<'_> ShrAssign<&'_ isize> for u32
impl<'_> ShrAssign<&'_ u128> for u32
impl<'_> ShrAssign<&'_ u16> for u32
impl<'_> ShrAssign<&'_ u32> for i16
impl<'_> ShrAssign<&'_ u32> for i32
impl<'_> ShrAssign<&'_ u32> for u32
impl<'_> ShrAssign<&'_ u32> for u128
impl<'_> ShrAssign<&'_ u32> for i64
impl<'_> ShrAssign<&'_ u32> for u16
impl<'_> ShrAssign<&'_ u32> for u8
impl<'_> ShrAssign<&'_ u32> for i8
impl<'_> ShrAssign<&'_ u32> for isize
impl<'_> ShrAssign<&'_ u32> for usize
impl<'_> ShrAssign<&'_ u32> for i128
impl<'_> ShrAssign<&'_ u32> for u64
impl<'_> ShrAssign<&'_ u64> for u32
impl<'_> ShrAssign<&'_ u8> for u32
impl<'_> ShrAssign<&'_ usize> for u32
impl ShrAssign<i128> for u32
impl ShrAssign<i16> for u32
impl ShrAssign<i32> for u32
impl ShrAssign<i64> for u32
impl ShrAssign<i8> for u32
impl ShrAssign<isize> for u32
impl ShrAssign<u128> for u32
impl ShrAssign<u16> for u32
impl ShrAssign<u32> for i16
impl ShrAssign<u32> for u64
impl ShrAssign<u32> for usize
impl ShrAssign<u32> for i32
impl ShrAssign<u32> for u128
impl ShrAssign<u32> for u32
impl ShrAssign<u32> for u8
impl ShrAssign<u32> for i128
impl ShrAssign<u32> for i8
impl ShrAssign<u32> for u16
impl ShrAssign<u32> for i64
impl ShrAssign<u32> for isize
impl ShrAssign<u64> for u32
impl ShrAssign<u8> for u32
impl ShrAssign<usize> for u32
impl SimdElement for u32
impl Step for u32
impl<'_> Sub<&'_ u32> for u32
impl<'_, '_> Sub<&'_ u32> for &'_ u32
impl Sub<u32> for u32
impl<'a> Sub<u32> for &'a u32
impl<'_> SubAssign<&'_ u32> for u32
impl SubAssign<u32> for u32
impl<'a> Sum<&'a u32> for u32
impl Sum<u32> for u32
impl TryFrom<i128> for u32
impl TryFrom<i16> for u32
impl TryFrom<i32> for u32
impl TryFrom<i64> for u32
impl TryFrom<i8> for u32
impl TryFrom<isize> for u32
impl TryFrom<u128> for u32
impl TryFrom<u32> for i16
impl TryFrom<u32> for i32
impl TryFrom<u32> for NonZeroU32
impl TryFrom<u32> for isize
impl TryFrom<u32> for usize
impl TryFrom<u32> for u8
impl TryFrom<u32> for char
impl TryFrom<u32> for i8
impl TryFrom<u32> for u16
impl TryFrom<u64> for u32
impl TryFrom<usize> for u32
impl UpperExp for u32
impl UpperHex for u32
impl<'_, '_> Add<&'_ u8> for &'_ u8
impl<'_> Add<&'_ u8> for u8
impl Add<u8> for u8
impl<'a> Add<u8> for &'a u8
impl<'_> AddAssign<&'_ u8> for u8
impl AddAssign<u8> for u8
impl AsciiExt for u8
impl Binary for u8
impl<'_> BitAnd<&'_ u8> for u8
impl<'_, '_> BitAnd<&'_ u8> for &'_ u8
impl BitAnd<u8> for u8
impl<'a> BitAnd<u8> for &'a u8
impl<'_> BitAndAssign<&'_ u8> for u8
impl BitAndAssign<u8> for u8
impl<'_> BitOr<&'_ u8> for u8
impl<'_, '_> BitOr<&'_ u8> for &'_ u8
impl BitOr<NonZeroU8> for u8
impl<'a> BitOr<u8> for &'a u8
impl BitOr<u8> for NonZeroU8
impl BitOr<u8> for u8
impl<'_> BitOrAssign<&'_ u8> for u8
impl BitOrAssign<u8> for u8
impl BitOrAssign<u8> for NonZeroU8
impl<'_, '_> BitXor<&'_ u8> for &'_ u8
impl<'_> BitXor<&'_ u8> for u8
impl BitXor<u8> for u8
impl<'a> BitXor<u8> for &'a u8
impl<'_> BitXorAssign<&'_ u8> for u8
impl BitXorAssign<u8> for u8
impl Clone for u8
impl Debug for u8
impl Default for u8
impl Display for u8
impl<'_, '_> Div<&'_ u8> for &'_ u8
impl<'_> Div<&'_ u8> for u8
impl Div<NonZeroU8> for u8
impl Div<u8> for u8
impl<'a> Div<u8> for &'a u8
impl<'_> DivAssign<&'_ u8> for u8
impl DivAssign<u8> for u8
impl From<NonZeroU8> for u8
impl From<bool> for u8
impl From<u8> for f32
impl From<u8> for i128
impl From<u8> for u32
impl From<u8> for char
impl From<u8> for isize
impl From<u8> for AtomicU8
impl From<u8> for f64
impl From<u8> for i64
impl From<u8> for u128
impl From<u8> for usize
impl From<u8> for u64
impl From<u8> for u16
impl From<u8> for i32
impl From<u8> for i16
impl FromStr for u8
impl Hash for u8
impl LowerExp for u8
impl LowerHex for u8
impl<'_, '_> Mul<&'_ u8> for &'_ u8
impl<'_> Mul<&'_ u8> for u8
impl<'a> Mul<u8> for &'a u8
impl Mul<u8> for u8
impl<'_> MulAssign<&'_ u8> for u8
impl MulAssign<u8> for u8
impl Not for u8
impl<'_> Not for &'_ u8
impl Octal for u8
impl Ord for u8
impl PartialEq<u8> for u8
impl PartialOrd<u8> for u8
impl<'a> Product<&'a u8> for u8
impl Product<u8> for u8
impl<'_> Rem<&'_ u8> for u8
impl<'_, '_> Rem<&'_ u8> for &'_ u8
impl Rem<NonZeroU8> for u8
impl<'a> Rem<u8> for &'a u8
impl Rem<u8> for u8
impl<'_> RemAssign<&'_ u8> for u8
impl RemAssign<u8> for u8
impl<'_> Shl<&'_ i128> for u8
impl<'_, '_> Shl<&'_ i128> for &'_ u8
impl<'_> Shl<&'_ i16> for u8
impl<'_, '_> Shl<&'_ i16> for &'_ u8
impl<'_, '_> Shl<&'_ i32> for &'_ u8
impl<'_> Shl<&'_ i32> for u8
impl<'_, '_> Shl<&'_ i64> for &'_ u8
impl<'_> Shl<&'_ i64> for u8
impl<'_> Shl<&'_ i8> for u8
impl<'_, '_> Shl<&'_ i8> for &'_ u8
impl<'_> Shl<&'_ isize> for u8
impl<'_, '_> Shl<&'_ isize> for &'_ u8
impl<'_> Shl<&'_ u128> for u8
impl<'_, '_> Shl<&'_ u128> for &'_ u8
impl<'_> Shl<&'_ u16> for u8
impl<'_, '_> Shl<&'_ u16> for &'_ u8
impl<'_> Shl<&'_ u32> for u8
impl<'_, '_> Shl<&'_ u32> for &'_ u8
impl<'_, '_> Shl<&'_ u64> for &'_ u8
impl<'_> Shl<&'_ u64> for u8
impl<'_, '_> Shl<&'_ u8> for &'_ i8
impl<'_> Shl<&'_ u8> for isize
impl<'_> Shl<&'_ u8> for i16
impl<'_> Shl<&'_ u8> for u128
impl<'_, '_> Shl<&'_ u8> for &'_ isize
impl<'_, '_> Shl<&'_ u8> for &'_ usize
impl<'_, '_> Shl<&'_ u8> for &'_ u8
impl<'_> Shl<&'_ u8> for i64
impl<'_> Shl<&'_ u8> for i8
impl<'_> Shl<&'_ u8> for usize
impl<'_> Shl<&'_ u8> for u32
impl<'_, '_> Shl<&'_ u8> for &'_ u16
impl<'_> Shl<&'_ u8> for u8
impl<'_, '_> Shl<&'_ u8> for &'_ u32
impl<'_, '_> Shl<&'_ u8> for &'_ i64
impl<'_, '_> Shl<&'_ u8> for &'_ i16
impl<'_, '_> Shl<&'_ u8> for &'_ u64
impl<'_, '_> Shl<&'_ u8> for &'_ i128
impl<'_> Shl<&'_ u8> for i32
impl<'_> Shl<&'_ u8> for u16
impl<'_, '_> Shl<&'_ u8> for &'_ u128
impl<'_> Shl<&'_ u8> for u64
impl<'_> Shl<&'_ u8> for i128
impl<'_, '_> Shl<&'_ u8> for &'_ i32
impl<'_> Shl<&'_ usize> for u8
impl<'_, '_> Shl<&'_ usize> for &'_ u8
impl Shl<i128> for u8
impl<'a> Shl<i128> for &'a u8
impl Shl<i16> for u8
impl<'a> Shl<i16> for &'a u8
impl Shl<i32> for u8
impl<'a> Shl<i32> for &'a u8
impl Shl<i64> for u8
impl<'a> Shl<i64> for &'a u8
impl Shl<i8> for u8
impl<'a> Shl<i8> for &'a u8
impl Shl<isize> for u8
impl<'a> Shl<isize> for &'a u8
impl<'a> Shl<u128> for &'a u8
impl Shl<u128> for u8
impl<'a> Shl<u16> for &'a u8
impl Shl<u16> for u8
impl Shl<u32> for u8
impl<'a> Shl<u32> for &'a u8
impl Shl<u64> for u8
impl<'a> Shl<u64> for &'a u8
impl Shl<u8> for isize
impl<'a> Shl<u8> for &'a usize
impl Shl<u8> for u8
impl Shl<u8> for i16
impl<'a> Shl<u8> for &'a i16
impl Shl<u8> for i64
impl Shl<u8> for i128
impl Shl<u8> for u128
impl<'a> Shl<u8> for &'a i128
impl<'a> Shl<u8> for &'a isize
impl Shl<u8> for i8
impl<'a> Shl<u8> for &'a i32
impl<'a> Shl<u8> for &'a u128
impl Shl<u8> for u32
impl<'a> Shl<u8> for &'a u64
impl Shl<u8> for u64
impl<'a> Shl<u8> for &'a u16
impl Shl<u8> for u16
impl<'a> Shl<u8> for &'a i8
impl<'a> Shl<u8> for &'a u8
impl Shl<u8> for usize
impl<'a> Shl<u8> for &'a i64
impl Shl<u8> for i32
impl<'a> Shl<u8> for &'a u32
impl<'a> Shl<usize> for &'a u8
impl Shl<usize> for u8
impl<'_> ShlAssign<&'_ i128> for u8
impl<'_> ShlAssign<&'_ i16> for u8
impl<'_> ShlAssign<&'_ i32> for u8
impl<'_> ShlAssign<&'_ i64> for u8
impl<'_> ShlAssign<&'_ i8> for u8
impl<'_> ShlAssign<&'_ isize> for u8
impl<'_> ShlAssign<&'_ u128> for u8
impl<'_> ShlAssign<&'_ u16> for u8
impl<'_> ShlAssign<&'_ u32> for u8
impl<'_> ShlAssign<&'_ u64> for u8
impl<'_> ShlAssign<&'_ u8> for isize
impl<'_> ShlAssign<&'_ u8> for i64
impl<'_> ShlAssign<&'_ u8> for u64
impl<'_> ShlAssign<&'_ u8> for usize
impl<'_> ShlAssign<&'_ u8> for i16
impl<'_> ShlAssign<&'_ u8> for i128
impl<'_> ShlAssign<&'_ u8> for u16
impl<'_> ShlAssign<&'_ u8> for u32
impl<'_> ShlAssign<&'_ u8> for u8
impl<'_> ShlAssign<&'_ u8> for u128
impl<'_> ShlAssign<&'_ u8> for i32
impl<'_> ShlAssign<&'_ u8> for i8
impl<'_> ShlAssign<&'_ usize> for u8
impl ShlAssign<i128> for u8
impl ShlAssign<i16> for u8
impl ShlAssign<i32> for u8
impl ShlAssign<i64> for u8
impl ShlAssign<i8> for u8
impl ShlAssign<isize> for u8
impl ShlAssign<u128> for u8
impl ShlAssign<u16> for u8
impl ShlAssign<u32> for u8
impl ShlAssign<u64> for u8
impl ShlAssign<u8> for u16
impl ShlAssign<u8> for i64
impl ShlAssign<u8> for i32
impl ShlAssign<u8> for u32
impl ShlAssign<u8> for i8
impl ShlAssign<u8> for u128
impl ShlAssign<u8> for usize
impl ShlAssign<u8> for i128
impl ShlAssign<u8> for isize
impl ShlAssign<u8> for i16
impl ShlAssign<u8> for u64
impl ShlAssign<u8> for u8
impl ShlAssign<usize> for u8
impl<'_> Shr<&'_ i128> for u8
impl<'_, '_> Shr<&'_ i128> for &'_ u8
impl<'_, '_> Shr<&'_ i16> for &'_ u8
impl<'_> Shr<&'_ i16> for u8
impl<'_, '_> Shr<&'_ i32> for &'_ u8
impl<'_> Shr<&'_ i32> for u8
impl<'_, '_> Shr<&'_ i64> for &'_ u8
impl<'_> Shr<&'_ i64> for u8
impl<'_, '_> Shr<&'_ i8> for &'_ u8
impl<'_> Shr<&'_ i8> for u8
impl<'_> Shr<&'_ isize> for u8
impl<'_, '_> Shr<&'_ isize> for &'_ u8
impl<'_, '_> Shr<&'_ u128> for &'_ u8
impl<'_> Shr<&'_ u128> for u8
impl<'_, '_> Shr<&'_ u16> for &'_ u8
impl<'_> Shr<&'_ u16> for u8
impl<'_, '_> Shr<&'_ u32> for &'_ u8
impl<'_> Shr<&'_ u32> for u8
impl<'_> Shr<&'_ u64> for u8
impl<'_, '_> Shr<&'_ u64> for &'_ u8
impl<'_, '_> Shr<&'_ u8> for &'_ i8
impl<'_, '_> Shr<&'_ u8> for &'_ i32
impl<'_> Shr<&'_ u8> for i32
impl<'_> Shr<&'_ u8> for u16
impl<'_> Shr<&'_ u8> for u32
impl<'_> Shr<&'_ u8> for isize
impl<'_, '_> Shr<&'_ u8> for &'_ u32
impl<'_, '_> Shr<&'_ u8> for &'_ i64
impl<'_> Shr<&'_ u8> for u64
impl<'_, '_> Shr<&'_ u8> for &'_ u8
impl<'_, '_> Shr<&'_ u8> for &'_ u16
impl<'_, '_> Shr<&'_ u8> for &'_ i128
impl<'_> Shr<&'_ u8> for i64
impl<'_, '_> Shr<&'_ u8> for &'_ usize
impl<'_, '_> Shr<&'_ u8> for &'_ u64
impl<'_> Shr<&'_ u8> for i16
impl<'_> Shr<&'_ u8> for u128
impl<'_, '_> Shr<&'_ u8> for &'_ u128
impl<'_> Shr<&'_ u8> for i128
impl<'_> Shr<&'_ u8> for usize
impl<'_, '_> Shr<&'_ u8> for &'_ i16
impl<'_> Shr<&'_ u8> for u8
impl<'_, '_> Shr<&'_ u8> for &'_ isize
impl<'_> Shr<&'_ u8> for i8
impl<'_> Shr<&'_ usize> for u8
impl<'_, '_> Shr<&'_ usize> for &'_ u8
impl<'a> Shr<i128> for &'a u8
impl Shr<i128> for u8
impl Shr<i16> for u8
impl<'a> Shr<i16> for &'a u8
impl<'a> Shr<i32> for &'a u8
impl Shr<i32> for u8
impl Shr<i64> for u8
impl<'a> Shr<i64> for &'a u8
impl Shr<i8> for u8
impl<'a> Shr<i8> for &'a u8
impl<'a> Shr<isize> for &'a u8
impl Shr<isize> for u8
impl<'a> Shr<u128> for &'a u8
impl Shr<u128> for u8
impl<'a> Shr<u16> for &'a u8
impl Shr<u16> for u8
impl Shr<u32> for u8
impl<'a> Shr<u32> for &'a u8
impl Shr<u64> for u8
impl<'a> Shr<u64> for &'a u8
impl Shr<u8> for usize
impl<'a> Shr<u8> for &'a i8
impl Shr<u8> for u64
impl Shr<u8> for isize
impl Shr<u8> for u16
impl<'a> Shr<u8> for &'a i128
impl<'a> Shr<u8> for &'a i16
impl Shr<u8> for i16
impl Shr<u8> for u128
impl<'a> Shr<u8> for &'a i32
impl Shr<u8> for i64
impl<'a> Shr<u8> for &'a u8
impl Shr<u8> for u8
impl Shr<u8> for i32
impl Shr<u8> for u32
impl<'a> Shr<u8> for &'a u64
impl<'a> Shr<u8> for &'a usize
impl Shr<u8> for i128
impl Shr<u8> for i8
impl<'a> Shr<u8> for &'a u16
impl<'a> Shr<u8> for &'a i64
impl<'a> Shr<u8> for &'a isize
impl<'a> Shr<u8> for &'a u128
impl<'a> Shr<u8> for &'a u32
impl<'a> Shr<usize> for &'a u8
impl Shr<usize> for u8
impl<'_> ShrAssign<&'_ i128> for u8
impl<'_> ShrAssign<&'_ i16> for u8
impl<'_> ShrAssign<&'_ i32> for u8
impl<'_> ShrAssign<&'_ i64> for u8
impl<'_> ShrAssign<&'_ i8> for u8
impl<'_> ShrAssign<&'_ isize> for u8
impl<'_> ShrAssign<&'_ u128> for u8
impl<'_> ShrAssign<&'_ u16> for u8
impl<'_> ShrAssign<&'_ u32> for u8
impl<'_> ShrAssign<&'_ u64> for u8
impl<'_> ShrAssign<&'_ u8> for i16
impl<'_> ShrAssign<&'_ u8> for i8
impl<'_> ShrAssign<&'_ u8> for isize
impl<'_> ShrAssign<&'_ u8> for u32
impl<'_> ShrAssign<&'_ u8> for i128
impl<'_> ShrAssign<&'_ u8> for usize
impl<'_> ShrAssign<&'_ u8> for i32
impl<'_> ShrAssign<&'_ u8> for u8
impl<'_> ShrAssign<&'_ u8> for u16
impl<'_> ShrAssign<&'_ u8> for i64
impl<'_> ShrAssign<&'_ u8> for u64
impl<'_> ShrAssign<&'_ u8> for u128
impl<'_> ShrAssign<&'_ usize> for u8
impl ShrAssign<i128> for u8
impl ShrAssign<i16> for u8
impl ShrAssign<i32> for u8
impl ShrAssign<i64> for u8
impl ShrAssign<i8> for u8
impl ShrAssign<isize> for u8
impl ShrAssign<u128> for u8
impl ShrAssign<u16> for u8
impl ShrAssign<u32> for u8
impl ShrAssign<u64> for u8
impl ShrAssign<u8> for u64
impl ShrAssign<u8> for u8
impl ShrAssign<u8> for u16
impl ShrAssign<u8> for i128
impl ShrAssign<u8> for i32
impl ShrAssign<u8> for usize
impl ShrAssign<u8> for u128
impl ShrAssign<u8> for i8
impl ShrAssign<u8> for i16
impl ShrAssign<u8> for i64
impl ShrAssign<u8> for u32
impl ShrAssign<u8> for isize
impl ShrAssign<usize> for u8
impl SimdElement for u8
impl Step for u8
impl<'_, '_> Sub<&'_ u8> for &'_ u8
impl<'_> Sub<&'_ u8> for u8
impl Sub<u8> for u8
impl<'a> Sub<u8> for &'a u8
impl<'_> SubAssign<&'_ u8> for u8
impl SubAssign<u8> for u8
impl<'a> Sum<&'a u8> for u8
impl Sum<u8> for u8
impl ToString for u8
impl TryFrom<char> for u8
impl TryFrom<i128> for u8
impl TryFrom<i16> for u8
impl TryFrom<i32> for u8
impl TryFrom<i64> for u8
impl TryFrom<i8> for u8
impl TryFrom<isize> for u8
impl TryFrom<u128> for u8
impl TryFrom<u16> for u8
impl TryFrom<u32> for u8
impl TryFrom<u64> for u8
impl TryFrom<u8> for i8
impl TryFrom<u8> for NonZeroU8
impl TryFrom<usize> for u8
impl UpperExp for u8
impl UpperHex for u8
impl<'_, '_> Add<&'_ usize> for &'_ usize
impl<'_> Add<&'_ usize> for usize
impl Add<usize> for usize
impl<'a> Add<usize> for &'a usize
impl<'_> AddAssign<&'_ usize> for usize
impl AddAssign<usize> for usize
impl Binary for usize
impl<'_> BitAnd<&'_ usize> for usize
impl<'_, '_> BitAnd<&'_ usize> for &'_ usize
impl<'a> BitAnd<usize> for &'a usize
impl BitAnd<usize> for usize
impl<'_> BitAndAssign<&'_ usize> for usize
impl BitAndAssign<usize> for usize
impl<'_> BitOr<&'_ usize> for usize
impl<'_, '_> BitOr<&'_ usize> for &'_ usize
impl BitOr<NonZeroUsize> for usize
impl BitOr<usize> for NonZeroUsize
impl BitOr<usize> for usize
impl<'a> BitOr<usize> for &'a usize
impl<'_> BitOrAssign<&'_ usize> for usize
impl BitOrAssign<usize> for usize
impl BitOrAssign<usize> for NonZeroUsize
impl<'_> BitXor<&'_ usize> for usize
impl<'_, '_> BitXor<&'_ usize> for &'_ usize
impl BitXor<usize> for usize
impl<'a> BitXor<usize> for &'a usize
impl<'_> BitXorAssign<&'_ usize> for usize
impl BitXorAssign<usize> for usize
impl Clone for usize
impl Debug for usize
impl Default for usize
impl Display for usize
impl<'_, '_> Div<&'_ usize> for &'_ usize
impl<'_> Div<&'_ usize> for usize
impl Div<NonZeroUsize> for usize
impl Div<usize> for usize
impl<'a> Div<usize> for &'a usize
impl<'_> DivAssign<&'_ usize> for usize
impl DivAssign<usize> for usize
impl From<NonZeroUsize> for usize
impl From<bool> for usize
impl From<u16> for usize
impl From<u8> for usize
impl From<usize> for AtomicUsize
impl FromStr for usize
impl Hash for usize
impl<T, A> Index<usize> for VecDeque<T, A> where    A: Allocator,
impl<T, A> IndexMut<usize> for VecDeque<T, A> where    A: Allocator,
impl LowerExp for usize
impl LowerHex for usize
impl<'_> Mul<&'_ usize> for usize
impl<'_, '_> Mul<&'_ usize> for &'_ usize
impl<'a> Mul<usize> for &'a usize
impl Mul<usize> for usize
impl<'_> MulAssign<&'_ usize> for usize
impl MulAssign<usize> for usize
impl Not for usize
impl<'_> Not for &'_ usize
impl Octal for usize
impl Ord for usize
impl PartialEq<usize> for usize
impl PartialOrd<usize> for usize
impl<'a> Product<&'a usize> for usize
impl Product<usize> for usize
impl<'_, '_> Rem<&'_ usize> for &'_ usize
impl<'_> Rem<&'_ usize> for usize
impl Rem<NonZeroUsize> for usize
impl Rem<usize> for usize
impl<'a> Rem<usize> for &'a usize
impl<'_> RemAssign<&'_ usize> for usize
impl RemAssign<usize> for usize
impl<'_> Shl<&'_ i128> for usize
impl<'_, '_> Shl<&'_ i128> for &'_ usize
impl<'_, '_> Shl<&'_ i16> for &'_ usize
impl<'_> Shl<&'_ i16> for usize
impl<'_, '_> Shl<&'_ i32> for &'_ usize
impl<'_> Shl<&'_ i32> for usize
impl<'_> Shl<&'_ i64> for usize
impl<'_, '_> Shl<&'_ i64> for &'_ usize
impl<'_> Shl<&'_ i8> for usize
impl<'_, '_> Shl<&'_ i8> for &'_ usize
impl<'_> Shl<&'_ isize> for usize
impl<'_, '_> Shl<&'_ isize> for &'_ usize
impl<'_> Shl<&'_ u128> for usize
impl<'_, '_> Shl<&'_ u128> for &'_ usize
impl<'_, '_> Shl<&'_ u16> for &'_ usize
impl<'_> Shl<&'_ u16> for usize
impl<'_, '_> Shl<&'_ u32> for &'_ usize
impl<'_> Shl<&'_ u32> for usize
impl<'_, '_> Shl<&'_ u64> for &'_ usize
impl<'_> Shl<&'_ u64> for usize
impl<'_, '_> Shl<&'_ u8> for &'_ usize
impl<'_> Shl<&'_ u8> for usize
impl<'_> Shl<&'_ usize> for Saturating<isize>
impl<'_> Shl<&'_ usize> for u128
impl<'_, '_> Shl<&'_ usize> for &'_ Wrapping<u8>
impl<'_> Shl<&'_ usize> for u16
impl<'_, '_> Shl<&'_ usize> for &'_ Saturating<usize>
impl<'_, '_> Shl<&'_ usize> for &'_ u64
impl<'_> Shl<&'_ usize> for i8
impl<'_> Shl<&'_ usize> for Wrapping<i64>
impl<'_, '_> Shl<&'_ usize> for &'_ Saturating<isize>
impl<'_> Shl<&'_ usize> for Wrapping<i128>
impl<'_> Shl<&'_ usize> for u64
impl<'_, '_> Shl<&'_ usize> for &'_ Wrapping<u32>
impl<'_, '_> Shl<&'_ usize> for &'_ u128
impl<'_, '_> Shl<&'_ usize> for &'_ Saturating<u64>
impl<'_, '_> Shl<&'_ usize> for &'_ Wrapping<usize>
impl<'_> Shl<&'_ usize> for i128
impl<'_, '_> Shl<&'_ usize> for &'_ Wrapping<isize>
impl<'_, '_> Shl<&'_ usize> for &'_ u16
impl<'_> Shl<&'_ usize> for Wrapping<u16>
impl<'_, '_> Shl<&'_ usize> for &'_ Wrapping<u128>
impl<'_> Shl<&'_ usize> for Wrapping<i32>
impl<'_> Shl<&'_ usize> for Saturating<i128>
impl<'_> Shl<&'_ usize> for i32
impl<'_> Shl<&'_ usize> for i16
impl<'_, '_> Shl<&'_ usize> for &'_ Wrapping<i64>
impl<'_> Shl<&'_ usize> for Saturating<u128>
impl<'_, '_> Shl<&'_ usize> for &'_ Saturating<i8>
impl<'_> Shl<&'_ usize> for Wrapping<i8>
impl<'_, '_> Shl<&'_ usize> for &'_ i64
impl<'_, '_> Shl<&'_ usize> for &'_ Wrapping<i32>
impl<'_, '_> Shl<&'_ usize> for &'_ Saturating<i32>
impl<'_> Shl<&'_ usize> for u8
impl<'_, '_> Shl<&'_ usize> for &'_ Saturating<u128>
impl<'_, '_> Shl<&'_ usize> for &'_ u8
impl<'_, '_> Shl<&'_ usize> for &'_ i8
impl<'_> Shl<&'_ usize> for Wrapping<isize>
impl<'_> Shl<&'_ usize> for Saturating<usize>
impl<'_, '_> Shl<&'_ usize> for &'_ u32
impl<'_> Shl<&'_ usize> for isize
impl<'_, '_> Shl<&'_ usize> for &'_ Saturating<u16>
impl<'_> Shl<&'_ usize> for Saturating<i8>
impl<'_> Shl<&'_ usize> for Saturating<u32>
impl<'_, '_> Shl<&'_ usize> for &'_ Wrapping<i8>
impl<'_> Shl<&'_ usize> for i64
impl<'_> Shl<&'_ usize> for u32
impl<'_> Shl<&'_ usize> for Wrapping<u64>
impl<'_, '_> Shl<&'_ usize> for &'_ isize
impl<'_> Shl<&'_ usize> for Saturating<u8>
impl<'_> Shl<&'_ usize> for Wrapping<u128>
impl<'_> Shl<&'_ usize> for Saturating<i32>
impl<'_, '_> Shl<&'_ usize> for &'_ Saturating<i16>
impl<'_> Shl<&'_ usize> for Wrapping<u32>
impl<'_, '_> Shl<&'_ usize> for &'_ i128
impl<'_, '_> Shl<&'_ usize> for &'_ Wrapping<u16>
impl<'_, '_> Shl<&'_ usize> for &'_ Wrapping<u64>
impl<'_, '_> Shl<&'_ usize> for &'_ Wrapping<i16>
impl<'_, '_> Shl<&'_ usize> for &'_ Wrapping<i128>
impl<'_, '_> Shl<&'_ usize> for &'_ Saturating<i64>
impl<'_, '_> Shl<&'_ usize> for &'_ Saturating<u32>
impl<'_> Shl<&'_ usize> for Wrapping<usize>
impl<'_> Shl<&'_ usize> for Saturating<u16>
impl<'_, '_> Shl<&'_ usize> for &'_ i16
impl<'_> Shl<&'_ usize> for Saturating<u64>
impl<'_, '_> Shl<&'_ usize> for &'_ Saturating<u8>
impl<'_> Shl<&'_ usize> for usize
impl<'_> Shl<&'_ usize> for Saturating<i64>
impl<'_, '_> Shl<&'_ usize> for &'_ Saturating<i128>
impl<'_, '_> Shl<&'_ usize> for &'_ usize
impl<'_> Shl<&'_ usize> for Wrapping<u8>
impl<'_> Shl<&'_ usize> for Wrapping<i16>
impl<'_, '_> Shl<&'_ usize> for &'_ i32
impl<'_> Shl<&'_ usize> for Saturating<i16>
impl<'a> Shl<i128> for &'a usize
impl Shl<i128> for usize
impl<'a> Shl<i16> for &'a usize
impl Shl<i16> for usize
impl Shl<i32> for usize
impl<'a> Shl<i32> for &'a usize
impl<'a> Shl<i64> for &'a usize
impl Shl<i64> for usize
impl Shl<i8> for usize
impl<'a> Shl<i8> for &'a usize
impl Shl<isize> for usize
impl<'a> Shl<isize> for &'a usize
impl<'a> Shl<u128> for &'a usize
impl Shl<u128> for usize
impl Shl<u16> for usize
impl<'a> Shl<u16> for &'a usize
impl Shl<u32> for usize
impl<'a> Shl<u32> for &'a usize
impl Shl<u64> for usize
impl<'a> Shl<u64> for &'a usize
impl<'a> Shl<u8> for &'a usize
impl Shl<u8> for usize
impl Shl<usize> for Saturating<i64>
impl<'a> Shl<usize> for &'a Wrapping<i128>
impl Shl<usize> for Wrapping<i32>
impl<'a> Shl<usize> for &'a Wrapping<i64>
impl<'a> Shl<usize> for &'a Wrapping<u64>
impl<'a> Shl<usize> for &'a i64
impl<'a> Shl<usize> for &'a i16
impl Shl<usize> for i32
impl<'a> Shl<usize> for &'a Wrapping<u8>
impl<'a> Shl<usize> for &'a i8
impl<'a> Shl<usize> for &'a Saturating<u16>
impl Shl<usize> for Saturating<i32>
impl Shl<usize> for Wrapping<i64>
impl Shl<usize> for u16
impl Shl<usize> for Saturating<i8>
impl<'a> Shl<usize> for &'a Wrapping<i8>
impl<'a> Shl<usize> for &'a usize
impl Shl<usize> for i16
impl<'a> Shl<usize> for &'a isize
impl<'a> Shl<usize> for &'a u8
impl<'a> Shl<usize> for &'a Saturating<u8>
impl Shl<usize> for i128
impl Shl<usize> for Saturating<u64>
impl Shl<usize> for Saturating<i16>
impl Shl<usize> for Wrapping<isize>
impl<'a> Shl<usize> for &'a Wrapping<i32>
impl Shl<usize> for Wrapping<u8>
impl<'a> Shl<usize> for &'a Wrapping<i16>
impl Shl<usize> for usize
impl Shl<usize> for Saturating<isize>
impl Shl<usize> for Saturating<u16>
impl Shl<usize> for Saturating<u128>
impl<'a> Shl<usize> for &'a u128
impl Shl<usize> for u32
impl Shl<usize> for Wrapping<usize>
impl Shl<usize> for Wrapping<i16>
impl Shl<usize> for u64
impl<'a> Shl<usize> for &'a i128
impl<'a> Shl<usize> for &'a Saturating<i16>
impl<'a> Shl<usize> for &'a i32
impl Shl<usize> for u128
impl Shl<usize> for u8
impl<'a> Shl<usize> for &'a Saturating<u128>
impl<'a> Shl<usize> for &'a u32
impl<'a> Shl<usize> for &'a u16
impl<'a> Shl<usize> for &'a Wrapping<isize>
impl<'a> Shl<usize> for &'a Saturating<usize>
impl Shl<usize> for Saturating<usize>
impl Shl<usize> for Saturating<i128>
impl Shl<usize> for isize
impl<'a> Shl<usize> for &'a Wrapping<u32>
impl Shl<usize> for i8
impl<'a> Shl<usize> for &'a Wrapping<u128>
impl Shl<usize> for Wrapping<u16>
impl Shl<usize> for Wrapping<i8>
impl<'a> Shl<usize> for &'a Saturating<i32>
impl<'a> Shl<usize> for &'a Saturating<isize>
impl Shl<usize> for Wrapping<u64>
impl<'a> Shl<usize> for &'a Wrapping<usize>
impl<'a> Shl<usize> for &'a u64
impl Shl<usize> for Wrapping<u32>
impl<'a> Shl<usize> for &'a Saturating<u32>
impl Shl<usize> for Wrapping<u128>
impl Shl<usize> for Saturating<u8>
impl<'a> Shl<usize> for &'a Saturating<i64>
impl<'a> Shl<usize> for &'a Saturating<i128>
impl Shl<usize> for i64
impl Shl<usize> for Saturating<u32>
impl<'a> Shl<usize> for &'a Wrapping<u16>
impl<'a> Shl<usize> for &'a Saturating<i8>
impl<'a> Shl<usize> for &'a Saturating<u64>
impl Shl<usize> for Wrapping<i128>
impl<'_> ShlAssign<&'_ i128> for usize
impl<'_> ShlAssign<&'_ i16> for usize
impl<'_> ShlAssign<&'_ i32> for usize
impl<'_> ShlAssign<&'_ i64> for usize
impl<'_> ShlAssign<&'_ i8> for usize
impl<'_> ShlAssign<&'_ isize> for usize
impl<'_> ShlAssign<&'_ u128> for usize
impl<'_> ShlAssign<&'_ u16> for usize
impl<'_> ShlAssign<&'_ u32> for usize
impl<'_> ShlAssign<&'_ u64> for usize
impl<'_> ShlAssign<&'_ u8> for usize
impl<'_> ShlAssign<&'_ usize> for Saturating<i128>
impl<'_> ShlAssign<&'_ usize> for Wrapping<u32>
impl<'_> ShlAssign<&'_ usize> for u32
impl<'_> ShlAssign<&'_ usize> for Wrapping<i64>
impl<'_> ShlAssign<&'_ usize> for Saturating<i64>
impl<'_> ShlAssign<&'_ usize> for Wrapping<u64>
impl<'_> ShlAssign<&'_ usize> for Wrapping<i32>
impl<'_> ShlAssign<&'_ usize> for Wrapping<u128>
impl<'_> ShlAssign<&'_ usize> for i32
impl<'_> ShlAssign<&'_ usize> for usize
impl<'_> ShlAssign<&'_ usize> for Wrapping<usize>
impl<'_> ShlAssign<&'_ usize> for u16
impl<'_> ShlAssign<&'_ usize> for Saturating<u64>
impl<'_> ShlAssign<&'_ usize> for i128
impl<'_> ShlAssign<&'_ usize> for Wrapping<i8>
impl<'_> ShlAssign<&'_ usize> for i16
impl<'_> ShlAssign<&'_ usize> for Wrapping<isize>
impl<'_> ShlAssign<&'_ usize> for Saturating<isize>
impl<'_> ShlAssign<&'_ usize> for u8
impl<'_> ShlAssign<&'_ usize> for Wrapping<i128>
impl<'_> ShlAssign<&'_ usize> for Saturating<usize>
impl<'_> ShlAssign<&'_ usize> for Saturating<i32>
impl<'_> ShlAssign<&'_ usize> for Saturating<u128>
impl<'_> ShlAssign<&'_ usize> for Wrapping<u8>
impl<'_> ShlAssign<&'_ usize> for i8
impl<'_> ShlAssign<&'_ usize> for i64
impl<'_> ShlAssign<&'_ usize> for isize
impl<'_> ShlAssign<&'_ usize> for Wrapping<u16>
impl<'_> ShlAssign<&'_ usize> for Wrapping<i16>
impl<'_> ShlAssign<&'_ usize> for Saturating<u32>
impl<'_> ShlAssign<&'_ usize> for u128
impl<'_> ShlAssign<&'_ usize> for Saturating<u16>
impl<'_> ShlAssign<&'_ usize> for u64
impl<'_> ShlAssign<&'_ usize> for Saturating<i16>
impl<'_> ShlAssign<&'_ usize> for Saturating<u8>
impl<'_> ShlAssign<&'_ usize> for Saturating<i8>
impl ShlAssign<i128> for usize
impl ShlAssign<i16> for usize
impl ShlAssign<i32> for usize
impl ShlAssign<i64> for usize
impl ShlAssign<i8> for usize
impl ShlAssign<isize> for usize
impl ShlAssign<u128> for usize
impl ShlAssign<u16> for usize
impl ShlAssign<u32> for usize
impl ShlAssign<u64> for usize
impl ShlAssign<u8> for usize
impl ShlAssign<usize> for Wrapping<u64>
impl ShlAssign<usize> for u64
impl ShlAssign<usize> for i16
impl ShlAssign<usize> for usize
impl ShlAssign<usize> for Saturating<u16>
impl ShlAssign<usize> for Saturating<u8>
impl ShlAssign<usize> for Wrapping<i64>
impl ShlAssign<usize> for Saturating<u32>
impl ShlAssign<usize> for Wrapping<i32>
impl ShlAssign<usize> for Saturating<i128>
impl ShlAssign<usize> for Saturating<i64>
impl ShlAssign<usize> for i32
impl ShlAssign<usize> for Wrapping<u8>
impl ShlAssign<usize> for Wrapping<i128>
impl ShlAssign<usize> for Wrapping<usize>
impl ShlAssign<usize> for i64
impl ShlAssign<usize> for Wrapping<u32>
impl ShlAssign<usize> for Saturating<isize>
impl ShlAssign<usize> for Saturating<u128>
impl ShlAssign<usize> for u128
impl ShlAssign<usize> for i8
impl ShlAssign<usize> for Saturating<u64>
impl ShlAssign<usize> for Saturating<usize>
impl ShlAssign<usize> for u32
impl ShlAssign<usize> for Wrapping<i8>
impl ShlAssign<usize> for Saturating<i8>
impl ShlAssign<usize> for isize
impl ShlAssign<usize> for i128
impl ShlAssign<usize> for Saturating<i32>
impl ShlAssign<usize> for Wrapping<i16>
impl ShlAssign<usize> for Wrapping<u128>
impl ShlAssign<usize> for u16
impl ShlAssign<usize> for Saturating<i16>
impl ShlAssign<usize> for Wrapping<isize>
impl ShlAssign<usize> for u8
impl ShlAssign<usize> for Wrapping<u16>
impl<'_> Shr<&'_ i128> for usize
impl<'_, '_> Shr<&'_ i128> for &'_ usize
impl<'_> Shr<&'_ i16> for usize
impl<'_, '_> Shr<&'_ i16> for &'_ usize
impl<'_> Shr<&'_ i32> for usize
impl<'_, '_> Shr<&'_ i32> for &'_ usize
impl<'_> Shr<&'_ i64> for usize
impl<'_, '_> Shr<&'_ i64> for &'_ usize
impl<'_> Shr<&'_ i8> for usize
impl<'_, '_> Shr<&'_ i8> for &'_ usize
impl<'_> Shr<&'_ isize> for usize
impl<'_, '_> Shr<&'_ isize> for &'_ usize
impl<'_> Shr<&'_ u128> for usize
impl<'_, '_> Shr<&'_ u128> for &'_ usize
impl<'_> Shr<&'_ u16> for usize
impl<'_, '_> Shr<&'_ u16> for &'_ usize
impl<'_, '_> Shr<&'_ u32> for &'_ usize
impl<'_> Shr<&'_ u32> for usize
impl<'_> Shr<&'_ u64> for usize
impl<'_, '_> Shr<&'_ u64> for &'_ usize
impl<'_> Shr<&'_ u8> for usize
impl<'_, '_> Shr<&'_ u8> for &'_ usize
impl<'_> Shr<&'_ usize> for u64
impl<'_> Shr<&'_ usize> for Wrapping<u32>
impl<'_> Shr<&'_ usize> for Wrapping<isize>
impl<'_, '_> Shr<&'_ usize> for &'_ Saturating<i8>
impl<'_, '_> Shr<&'_ usize> for &'_ u128
impl<'_, '_> Shr<&'_ usize> for &'_ Saturating<isize>
impl<'_> Shr<&'_ usize> for Wrapping<u64>
impl<'_, '_> Shr<&'_ usize> for &'_ Saturating<i32>
impl<'_, '_> Shr<&'_ usize> for &'_ Wrapping<u32>
impl<'_> Shr<&'_ usize> for Saturating<isize>
impl<'_> Shr<&'_ usize> for Saturating<i32>
impl<'_, '_> Shr<&'_ usize> for &'_ Wrapping<i128>
impl<'_, '_> Shr<&'_ usize> for &'_ i8
impl<'_, '_> Shr<&'_ usize> for &'_ Wrapping<usize>
impl<'_> Shr<&'_ usize> for u32
impl<'_> Shr<&'_ usize> for u16
impl<'_, '_> Shr<&'_ usize> for &'_ Wrapping<u8>
impl<'_, '_> Shr<&'_ usize> for &'_ Wrapping<u64>
impl<'_, '_> Shr<&'_ usize> for &'_ Wrapping<i16>
impl<'_> Shr<&'_ usize> for Wrapping<i128>
impl<'_> Shr<&'_ usize> for i64
impl<'_, '_> Shr<&'_ usize> for &'_ Wrapping<isize>
impl<'_, '_> Shr<&'_ usize> for &'_ isize
impl<'_, '_> Shr<&'_ usize> for &'_ u32
impl<'_> Shr<&'_ usize> for Saturating<i16>
impl<'_, '_> Shr<&'_ usize> for &'_ u16
impl<'_> Shr<&'_ usize> for Wrapping<u16>
impl<'_> Shr<&'_ usize> for Saturating<i128>
impl<'_, '_> Shr<&'_ usize> for &'_ Saturating<u32>
impl<'_> Shr<&'_ usize> for Saturating<i8>
impl<'_> Shr<&'_ usize> for i8
impl<'_, '_> Shr<&'_ usize> for &'_ Wrapping<i32>
impl<'_, '_> Shr<&'_ usize> for &'_ Saturating<usize>
impl<'_> Shr<&'_ usize> for Wrapping<i64>
impl<'_> Shr<&'_ usize> for Wrapping<i8>
impl<'_> Shr<&'_ usize> for i32
impl<'_> Shr<&'_ usize> for i128
impl<'_> Shr<&'_ usize> for Wrapping<u128>
impl<'_, '_> Shr<&'_ usize> for &'_ i64
impl<'_> Shr<&'_ usize> for usize
impl<'_, '_> Shr<&'_ usize> for &'_ Wrapping<i64>
impl<'_, '_> Shr<&'_ usize> for &'_ Saturating<u128>
impl<'_> Shr<&'_ usize> for Saturating<u16>
impl<'_, '_> Shr<&'_ usize> for &'_ Wrapping<i8>
impl<'_> Shr<&'_ usize> for Saturating<usize>
impl<'_, '_> Shr<&'_ usize> for &'_ usize
impl<'_, '_> Shr<&'_ usize> for &'_ i32
impl<'_, '_> Shr<&'_ usize> for &'_ i128
impl<'_> Shr<&'_ usize> for Wrapping<i16>
impl<'_> Shr<&'_ usize> for u8
impl<'_, '_> Shr<&'_ usize> for &'_ Saturating<i128>
impl<'_, '_> Shr<&'_ usize> for &'_ Saturating<u8>
impl<'_, '_> Shr<&'_ usize> for &'_ Saturating<i64>
impl<'_> Shr<&'_ usize> for u128
impl<'_, '_> Shr<&'_ usize> for &'_ u64
impl<'_, '_> Shr<&'_ usize> for &'_ i16
impl<'_, '_> Shr<&'_ usize> for &'_ Wrapping<u16>
impl<'_> Shr<&'_ usize> for Wrapping<u8>
impl<'_> Shr<&'_ usize> for Saturating<u8>
impl<'_> Shr<&'_ usize> for i16
impl<'_> Shr<&'_ usize> for Saturating<u64>
impl<'_> Shr<&'_ usize> for Saturating<u128>
impl<'_> Shr<&'_ usize> for Wrapping<i32>
impl<'_> Shr<&'_ usize> for Wrapping<usize>
impl<'_, '_> Shr<&'_ usize> for &'_ Saturating<u16>
impl<'_> Shr<&'_ usize> for isize
impl<'_, '_> Shr<&'_ usize> for &'_ Saturating<u64>
impl<'_> Shr<&'_ usize> for Saturating<i64>
impl<'_, '_> Shr<&'_ usize> for &'_ Saturating<i16>
impl<'_, '_> Shr<&'_ usize> for &'_ u8
impl<'_, '_> Shr<&'_ usize> for &'_ Wrapping<u128>
impl<'_> Shr<&'_ usize> for Saturating<u32>
impl Shr<i128> for usize
impl<'a> Shr<i128> for &'a usize
impl Shr<i16> for usize
impl<'a> Shr<i16> for &'a usize
impl<'a> Shr<i32> for &'a usize
impl Shr<i32> for usize
impl<'a> Shr<i64> for &'a usize
impl Shr<i64> for usize
impl<'a> Shr<i8> for &'a usize
impl Shr<i8> for usize
impl<'a> Shr<isize> for &'a usize
impl Shr<isize> for usize
impl<'a> Shr<u128> for &'a usize
impl Shr<u128> for usize
impl Shr<u16> for usize
impl<'a> Shr<u16> for &'a usize
impl Shr<u32> for usize
impl<'a> Shr<u32> for &'a usize
impl<'a> Shr<u64> for &'a usize
impl Shr<u64> for usize
impl Shr<u8> for usize
impl<'a> Shr<u8> for &'a usize
impl Shr<usize> for Wrapping<u32>
impl Shr<usize> for Wrapping<i8>
impl<'a> Shr<usize> for &'a Wrapping<u128>
impl<'a> Shr<usize> for &'a i8
impl Shr<usize> for isize
impl<'a> Shr<usize> for &'a Wrapping<u64>
impl<'a> Shr<usize> for &'a Saturating<usize>
impl<'a> Shr<usize> for &'a Wrapping<i64>
impl Shr<usize> for Saturating<i64>
impl Shr<usize> for u128
impl Shr<usize> for Wrapping<i64>
impl<'a> Shr<usize> for &'a Wrapping<u32>
impl Shr<usize> for i16
impl Shr<usize> for Saturating<i16>
impl Shr<usize> for Saturating<i32>
impl Shr<usize> for i32
impl<'a> Shr<usize> for &'a Saturating<i32>
impl<'a> Shr<usize> for &'a Saturating<i16>
impl Shr<usize> for Wrapping<u16>
impl Shr<usize> for Wrapping<u8>
impl<'a> Shr<usize> for &'a u128
impl<'a> Shr<usize> for &'a Saturating<i8>
impl<'a> Shr<usize> for &'a Saturating<isize>
impl Shr<usize> for Wrapping<u128>
impl<'a> Shr<usize> for &'a i64
impl Shr<usize> for Wrapping<u64>
impl Shr<usize> for Wrapping<isize>
impl Shr<usize> for Saturating<u128>
impl Shr<usize> for u16
impl<'a> Shr<usize> for &'a u8
impl<'a> Shr<usize> for &'a Saturating<u128>
impl Shr<usize> for u64
impl<'a> Shr<usize> for &'a u16
impl<'a> Shr<usize> for &'a Wrapping<u8>
impl<'a> Shr<usize> for &'a Wrapping<isize>
impl<'a> Shr<usize> for &'a usize
impl Shr<usize> for Saturating<isize>
impl Shr<usize> for Wrapping<usize>
impl Shr<usize> for Saturating<u16>
impl<'a> Shr<usize> for &'a i128
impl Shr<usize> for usize
impl Shr<usize> for Saturating<i128>
impl Shr<usize> for Wrapping<i128>
impl Shr<usize> for Wrapping<i16>
impl Shr<usize> for i8
impl<'a> Shr<usize> for &'a Saturating<u64>
impl<'a> Shr<usize> for &'a Wrapping<i128>
impl Shr<usize> for u32
impl<'a> Shr<usize> for &'a Wrapping<i16>
impl<'a> Shr<usize> for &'a Saturating<u32>
impl<'a> Shr<usize> for &'a u32
impl<'a> Shr<usize> for &'a i32
impl Shr<usize> for Saturating<u8>
impl<'a> Shr<usize> for &'a i16
impl Shr<usize> for i128
impl<'a> Shr<usize> for &'a Wrapping<i8>
impl<'a> Shr<usize> for &'a Wrapping<u16>
impl<'a> Shr<usize> for &'a Saturating<u8>
impl<'a> Shr<usize> for &'a isize
impl<'a> Shr<usize> for &'a Saturating<i64>
impl Shr<usize> for Saturating<u64>
impl<'a> Shr<usize> for &'a Wrapping<usize>
impl Shr<usize> for Saturating<u32>
impl<'a> Shr<usize> for &'a Saturating<u16>
impl Shr<usize> for u8
impl Shr<usize> for Saturating<i8>
impl Shr<usize> for i64
impl<'a> Shr<usize> for &'a u64
impl Shr<usize> for Wrapping<i32>
impl Shr<usize> for Saturating<usize>
impl<'a> Shr<usize> for &'a Wrapping<i32>
impl<'a> Shr<usize> for &'a Saturating<i128>
impl<'_> ShrAssign<&'_ i128> for usize
impl<'_> ShrAssign<&'_ i16> for usize
impl<'_> ShrAssign<&'_ i32> for usize
impl<'_> ShrAssign<&'_ i64> for usize
impl<'_> ShrAssign<&'_ i8> for usize
impl<'_> ShrAssign<&'_ isize> for usize
impl<'_> ShrAssign<&'_ u128> for usize
impl<'_> ShrAssign<&'_ u16> for usize
impl<'_> ShrAssign<&'_ u32> for usize
impl<'_> ShrAssign<&'_ u64> for usize
impl<'_> ShrAssign<&'_ u8> for usize
impl<'_> ShrAssign<&'_ usize> for Saturating<i64>
impl<'_> ShrAssign<&'_ usize> for u8
impl<'_> ShrAssign<&'_ usize> for i16
impl<'_> ShrAssign<&'_ usize> for Saturating<u128>
impl<'_> ShrAssign<&'_ usize> for i32
impl<'_> ShrAssign<&'_ usize> for Saturating<u8>
impl<'_> ShrAssign<&'_ usize> for u64
impl<'_> ShrAssign<&'_ usize> for Saturating<u32>
impl<'_> ShrAssign<&'_ usize> for Saturating<i16>
impl<'_> ShrAssign<&'_ usize> for Saturating<u64>
impl<'_> ShrAssign<&'_ usize> for u128
impl<'_> ShrAssign<&'_ usize> for Saturating<usize>
impl<'_> ShrAssign<&'_ usize> for Saturating<i128>
impl<'_> ShrAssign<&'_ usize> for Wrapping<i32>
impl<'_> ShrAssign<&'_ usize> for u16
impl<'_> ShrAssign<&'_ usize> for Wrapping<i64>
impl<'_> ShrAssign<&'_ usize> for i64
impl<'_> ShrAssign<&'_ usize> for Saturating<isize>
impl<'_> ShrAssign<&'_ usize> for Wrapping<i16>
impl<'_> ShrAssign<&'_ usize> for Wrapping<i8>
impl<'_> ShrAssign<&'_ usize> for Wrapping<usize>
impl<'_> ShrAssign<&'_ usize> for u32
impl<'_> ShrAssign<&'_ usize> for isize
impl<'_> ShrAssign<&'_ usize> for i8
impl<'_> ShrAssign<&'_ usize> for Wrapping<u8>
impl<'_> ShrAssign<&'_ usize> for Wrapping<u32>
impl<'_> ShrAssign<&'_ usize> for Wrapping<isize>
impl<'_> ShrAssign<&'_ usize> for Saturating<u16>
impl<'_> ShrAssign<&'_ usize> for Saturating<i32>
impl<'_> ShrAssign<&'_ usize> for usize
impl<'_> ShrAssign<&'_ usize> for i128
impl<'_> ShrAssign<&'_ usize> for Saturating<i8>
impl<'_> ShrAssign<&'_ usize> for Wrapping<u16>
impl<'_> ShrAssign<&'_ usize> for Wrapping<u128>
impl<'_> ShrAssign<&'_ usize> for Wrapping<u64>
impl<'_> ShrAssign<&'_ usize> for Wrapping<i128>
impl ShrAssign<i128> for usize
impl ShrAssign<i16> for usize
impl ShrAssign<i32> for usize
impl ShrAssign<i64> for usize
impl ShrAssign<i8> for usize
impl ShrAssign<isize> for usize
impl ShrAssign<u128> for usize
impl ShrAssign<u16> for usize
impl ShrAssign<u32> for usize
impl ShrAssign<u64> for usize
impl ShrAssign<u8> for usize
impl ShrAssign<usize> for Wrapping<u128>
impl ShrAssign<usize> for Saturating<i128>
impl ShrAssign<usize> for isize
impl ShrAssign<usize> for Saturating<i32>
impl ShrAssign<usize> for u8
impl ShrAssign<usize> for Wrapping<u16>
impl ShrAssign<usize> for i16
impl ShrAssign<usize> for Wrapping<isize>
impl ShrAssign<usize> for Wrapping<u8>
impl ShrAssign<usize> for u32
impl ShrAssign<usize> for u128
impl ShrAssign<usize> for Saturating<i16>
impl ShrAssign<usize> for Wrapping<i64>
impl ShrAssign<usize> for i32
impl ShrAssign<usize> for Wrapping<u64>
impl ShrAssign<usize> for usize
impl ShrAssign<usize> for Wrapping<usize>
impl ShrAssign<usize> for i64
impl ShrAssign<usize> for u64
impl ShrAssign<usize> for i128
impl ShrAssign<usize> for Saturating<u128>
impl ShrAssign<usize> for Saturating<i64>
impl ShrAssign<usize> for Wrapping<i32>
impl ShrAssign<usize> for Saturating<u32>
impl ShrAssign<usize> for Saturating<usize>
impl ShrAssign<usize> for Saturating<isize>
impl ShrAssign<usize> for u16
impl ShrAssign<usize> for Wrapping<i128>
impl ShrAssign<usize> for Saturating<u16>
impl ShrAssign<usize> for Wrapping<i16>
impl ShrAssign<usize> for Saturating<u64>
impl ShrAssign<usize> for Wrapping<i8>
impl ShrAssign<usize> for Saturating<u8>
impl ShrAssign<usize> for i8
impl ShrAssign<usize> for Wrapping<u32>
impl ShrAssign<usize> for Saturating<i8>
impl SimdElement for usize
impl<T> SliceIndex<[T]> for usize
impl Step for usize
impl<'_, '_> Sub<&'_ usize> for &'_ usize
impl<'_> Sub<&'_ usize> for usize
impl Sub<usize> for usize
impl<'a> Sub<usize> for &'a usize
impl<'_> SubAssign<&'_ usize> for usize
impl SubAssign<usize> for usize
impl<'a> Sum<&'a usize> for usize
impl Sum<usize> for usize
impl TryFrom<i128> for usize
impl TryFrom<i16> for usize
impl TryFrom<i32> for usize
impl TryFrom<i64> for usize
impl TryFrom<i8> for usize
impl TryFrom<isize> for usize
impl TryFrom<u128> for usize
impl TryFrom<u32> for usize
impl TryFrom<u64> for usize
impl TryFrom<usize> for i16
impl TryFrom<usize> for NonZeroUsize
impl TryFrom<usize> for u16
impl TryFrom<usize> for u64
impl TryFrom<usize> for i128
impl TryFrom<usize> for u8
impl TryFrom<usize> for u32
impl TryFrom<usize> for isize
impl TryFrom<usize> for u128
impl TryFrom<usize> for i32
impl TryFrom<usize> for i64
impl TryFrom<usize> for i8
impl UpperExp for usize
impl UpperHex for usize