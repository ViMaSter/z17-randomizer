use std::collections::HashSet;
use crate::filler_item::FillerItem;
use crate::filler_item::FillerItem::*;

#[derive(Clone, Default, Eq, PartialEq)]
pub struct Progress {
    items: HashSet<FillerItem>,
}

impl Progress {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn add_item(&mut self, item: FillerItem) {
        self.items.insert(item);
    }

    pub fn difference(&self, other: &Progress) -> HashSet<FillerItem> {
        let mut new_items : HashSet<FillerItem> = HashSet::new();

        for item in &self.items {
            if !other.items.contains(&item) {
                new_items.insert(*item);
            }
        }

        new_items
    }

    fn has(&self, item: FillerItem) -> bool {
        self.items.contains(&item)
    }

    fn has_either(&self, item1: FillerItem, item2: FillerItem) -> bool {
        self.items.contains(&item1) || self.items.contains(&item2)
    }

    fn has_both(&self, item1: FillerItem, item2: FillerItem) -> bool {
        self.items.contains(&item1) && self.items.contains(&item2)
    }

    fn has_any(&self, items: &[FillerItem]) -> bool {
        for item in items {
            if self.has(*item) {
                return true;
            }
        }

        false
    }

    fn has_amount(&self, amount: u8, items: &[FillerItem]) -> bool {
        self.count(items) >= amount
    }

    fn count(&self, items: &[FillerItem]) -> u8 {
        let mut sum: u8 = 0;
        for item in items {
            if self.clone().has(*item) {
                sum += 1;
            }
        }

        sum
    }

    pub fn has_bow(&self) -> bool {
        self.has_either(Bow01, Bow02)
    }

    pub fn has_boomerang(&self) -> bool {
        self.has_either(Boomerang01, Boomerang02)
    }

    pub fn has_hookshot(&self) -> bool {
        self.has_either(Hookshot01, Hookshot02)
    }

    pub fn has_bombs(&self) -> bool {
        self.has_either(Bombs01, Bombs02)
    }

    pub fn has_nice_bombs(&self) -> bool {
        self.has_both(Bombs01, Bombs02) // TODO need to incorporate Maiamai checks
    }

    pub fn has_fire_rod(&self) -> bool {
        self.has_either(FireRod01, FireRod02)
    }

    pub fn has_ice_rod(&self) -> bool {
        self.has_either(IceRod01, IceRod02)
    }

    pub fn has_hammer(&self) -> bool {
        self.has_either(Hammer01, Hammer02)
    }


    pub fn has_scoot_fruit(&self) -> bool {
        self.has(ScootFruit)
    }

    pub fn has_lamp(&self) -> bool {
        self.has_either(Lamp01, Lamp02)
    }

    pub fn has_fire_source(&self) -> bool {
        self.has_any(&[Lamp01, Lamp02, FireRod01, FireRod02])
    }

    pub fn can_extinguish_torches(&self) -> bool {
        self.has_any(&[Sword01, Sword02, Sword03, Sword04, Bombs01, Bombs02, IceRod01, IceRod02, TornadoRod01, TornadoRod02])
    }

    pub fn has_bell(&self) -> bool {
        self.has(Bell)
    }

    pub fn has_net(&self) -> bool {
        self.has_either(Lamp01, Lamp02)
    }

    pub fn has_stamina_scroll(&self) -> bool {
        self.has(StaminaScroll)
    }

    pub fn has_bottle(&self) -> bool {
        self.has_any(&[Bottle01, Bottle02, Bottle03, Bottle04, Bottle05])
    }

    pub fn has_sand_rod(&self) -> bool {
        self.has_either(SandRod01, SandRod02)
    }

    pub fn has_tornado_rod(&self) -> bool {
        self.has_either(TornadoRod01, TornadoRod02)
    }

    pub fn has_boots(&self) -> bool {
        self.has(PegasusBoots)
    }

    pub fn has_power_glove(&self) -> bool {
        self.has_either(Glove01, Glove02)
    }

    pub fn has_titans_mitt(&self) -> bool {
        self.has_both(Glove01, Glove02)
    }

    pub fn has_flippers(&self) -> bool {
        self.has(Flippers)
    }

    pub fn can_merge(&self) -> bool {
        self.has_either(RaviosBracelet01, RaviosBracelet02) // TODO change this to require both
    }

    // pub fn has_maiamai(self, amount: u8) -> bool { // TODO maiamai everything
    //     self.maiamai >= amount
    // }

    pub fn has_master_ore(&self, amount: u8) -> bool {
        self.has_amount(amount, &[OreRed, OreGreen, OreBlue, OreYellow])
    }

    pub fn has_smooth_gem(&self) -> bool {
        self.has(SmoothGem)
    }

    pub fn has_letter_in_a_bottle(&self) -> bool {
        self.has(LetterInABottle)
    }

    pub fn has_premium_milk(&self) -> bool {
        self.has(PremiumMilk)
    }

    pub fn has_gold_bee(&self) -> bool {
        self.has(GoldBee)
    }

    pub fn has_sword(&self) -> bool {
        self.has_any(&[Sword01, Sword02, Sword03, Sword04])
    }

    pub fn has_master_sword(&self) -> bool {
        self.has_amount(2, &[Sword01, Sword02, Sword03, Sword04])
    }

    pub fn has_great_spin(&self) -> bool {
        self.has(GreatSpin)
    }

    pub fn can_attack(&self) -> bool {
        self.has_any(&[
            Sword01, Sword02, Sword03, Sword04,
            Bow01, Bow02,
            Bombs01, Bombs02,
            FireRod01, FireRod02,
            IceRod01, IceRod02,
            Hammer01, Hammer02
        ])
    }

    pub fn can_hit_switch(&self) -> bool {
        self.has_any(&[
            Sword01, Sword02, Sword03, Sword04,
            Bow01, Bow02,
            Boomerang01, Boomerang02,
            Hookshot01, Hookshot02,
            Bombs01, Bombs02,
            IceRod01, IceRod02,
            Hammer01, Hammer02
        ])
    }

    pub fn can_hit_far_switch(&self) -> bool {
        self.has_any(&[
            Bow01, Bow02,
            Boomerang01, Boomerang02,
            Hookshot01, Hookshot02,
            Bombs01, Bombs02
        ])
    }

    pub fn can_hit_shielded_switch(&self) -> bool {
        self.has_any(&[
            Sword01, Sword02, Sword03, Sword04,
            Bow01, Bow02,
            Boomerang01, Boomerang02,
            Hookshot01, Hookshot02,
            Bombs01, Bombs02,
            Hammer01, Hammer02
        ])
    }

    pub fn can_hit_hog_1f_switch(&self) -> bool {
        self.can_hit_far_switch() || self.has_ice_rod() || (self.can_merge() && (self.has_sword() || self.has_hammer()))
    }

    pub fn has_sanctuary_key(&self) -> bool {
        self.has(HyruleSanctuaryKey)
    }

    pub fn has_lorule_sanctuary_key(&self) -> bool {
        self.has(LoruleSanctuaryKey)
    }

    pub fn has_eastern_keys(&self, amount: u8) -> bool {
        self.has_amount(amount, &[EasternKeySmall01, EasternKeySmall02])
    }

    pub fn has_eastern_big_key(&self) -> bool {
        self.has(EasternKeyBig)
    }

    pub fn can_defeat_yuga(&self) -> bool {
        self.has_bow() || self.has_bombs()
            || ((self.has_boomerang() || self.has_hookshot()) && self.can_attack())
    }

    pub fn has_gales_keys(&self, amount: u8) -> bool {
        self.has_amount(amount, &[GalesKeySmall01, GalesKeySmall02, GalesKeySmall03, GalesKeySmall04])
    }

    pub fn has_gales_big_key(&self) -> bool {
        self.has(GalesKeyBig)
    }

    pub fn can_defeat_margomill(&self) -> bool {
        self.has_tornado_rod() && (
            self.has_sword()
                || self.has_bow()
                || self.has_bombs()
                || self.has_fire_rod()
                || self.has_hammer()
        )
    }

    pub fn has_hera_keys(&self, amount: u8) -> bool {
        self.has_amount(amount, &[HeraKeySmall01, HeraKeySmall02])
    }

    pub fn has_hera_big_key(&self) -> bool {
        self.has(HeraKeyBig)
    }

    pub fn can_defeat_moldorm(&self) -> bool {
        self.has_hammer()
    }

    pub fn has_dark_keys(&self, amount: u8) -> bool {
        self.has_amount(amount, &[DarkKeySmall01, DarkKeySmall02, DarkKeySmall03, DarkKeySmall04])
    }

    pub fn has_dark_big_key(&self) -> bool {
        self.has(DarkKeyBig)
    }

    pub fn can_defeat_gemasaur(&self) -> bool {
        self.has_bombs() && self.has_fire_source()
    }

    pub fn has_swamp_keys(&self, amount: u8) -> bool {
        self.has_amount(amount, &[SwampKeySmall01, SwampKeySmall02, SwampKeySmall03, SwampKeySmall04])
    }

    pub fn has_swamp_big_key(&self) -> bool {
        self.has(SwampKeyBig)
    }

    pub fn can_defeat_arrgus(&self) -> bool {
        self.has_hookshot() && self.can_attack()
    }

    pub fn has_skull_keys(&self, amount: u8) -> bool {
        self.has_amount(amount, &[SkullKeySmall01, SkullKeySmall02, SkullKeySmall03])
    }

    pub fn has_skull_big_key(&self) -> bool {
        self.has(SkullKeyBig)
    }

    pub fn can_defeat_knucklemaster(&self) -> bool {
        self.can_merge()
            && (self.has_sword()
            || self.has_bombs()
            || self.has_fire_rod()
            || self.has_ice_rod()
            || self.has_hammer())
    }

    pub fn has_thieves_key(&self) -> bool {
        self.has(ThievesKeySmall)
    }

    pub fn has_thieves_big_key(&self) -> bool {
        self.has(ThievesKeyBig)
    }

    pub fn can_defeat_stalblind(&self) -> bool {
        self.can_merge() && self.can_attack()
    }

    pub fn has_ice_keys(&self, amount: u8) -> bool {
        self.has_amount(amount, &[IceKeySmall01, IceKeySmall02, IceKeySmall03])
    }

    pub fn has_ice_big_key(&self) -> bool {
        self.has(IceKeyBig)
    }

    pub fn can_defeat_dharkstare(&self) -> bool {
        self.has_fire_rod()
    }

    pub fn has_desert_keys(&self, amount: u8) -> bool {
        self.has_amount(amount, &[DesertKeySmall01, DesertKeySmall02, DesertKeySmall03, DesertKeySmall04, DesertKeySmall05])
    }

    pub fn has_desert_big_key(&self) -> bool {
        self.has(DesertKeyBig)
    }

    pub fn can_defeat_zaganaga(&self) -> bool {
        self.has_sand_rod() && self.can_attack()
    }

    pub fn has_turtle_keys(&self, amount: u8) -> bool {
        self.has_amount(amount, &[TurtleKeySmall01, TurtleKeySmall02, TurtleKeySmall03])
    }

    pub fn has_turtle_big_key(&self) -> bool {
        self.has(TurtleKeyBig)
    }

    pub fn can_defeat_grinexx(&self) -> bool {
        self.has_ice_rod()
    }

    pub fn has_lorule_keys(&self, amount: u8) -> bool {
        self.has_amount(amount, &[LoruleCastleKeySmall01, LoruleCastleKeySmall02, LoruleCastleKeySmall03, LoruleCastleKeySmall04, LoruleCastleKeySmall05])
    }

    pub fn can_defeat_yuganon(&self) -> bool {
        self.can_attack() && self.can_merge() && self.has(BowOfLight)
    }

    // Events ------------------------------------------------

    pub fn has_opened_stylish_womans_house(&self) -> bool {
        self.has(StylishWomansHouseOpen)
    }

    pub fn has_skull_eye_right(&self) -> bool {
        self.has(SkullEyeRight)
    }

    pub fn has_skull_eyes(&self) -> bool {
        self.has_both(SkullEyeLeft, SkullEyeRight)
    }

    pub fn has_bomb_flower(&self) -> bool {
        self.has(BigBombFlower)
    }

    pub fn has_pendant_of_courage(&self) -> bool {
        self.has(PendantOfCourage)
    }

    pub fn has_all_pendants(&self) -> bool {
        self.has(PendantOfCourage) && self.has(PendantOfWisdom) && self.has(PendantOfPower)
    }

    pub fn has_sage_osfala(&self) -> bool {
        self.has(SageOsfala)
    }

    pub fn has_all_sages(&self) -> bool {
        self.has(SageGulley)
            && self.has(SageOren)
            && self.has(SageSeres)
            && self.has(SageOsfala)
            && self.has(SageRosso)
            && self.has(SageIrene)
            && self.has(SageImpa)
    }

    pub fn can_reach_hilda_barrier(&self) -> bool { // TODO incorporate
        self.has(AccessHildaBarrier)
    }
}