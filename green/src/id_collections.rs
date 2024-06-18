pub mod area_id_collection {
    pub const TOKYO: &str = "13";
    pub const KANAGAWA: &str = "14";
    pub const CHIBA: &str = "12";
    pub const SAITAMA: &str = "11";
    pub const IBARAKI: &str = "8";
    pub const TOCHIGI: &str = "9";
    pub const GUNMA: &str = "10";
    pub const HOKKAIDO: &str = "1";
    pub const AOMORI: &str = "2";
    pub const IWATE: &str = "3";
    pub const MIYAGI: &str = "4";
    pub const AKITA: &str = "5";
    pub const YAMAGATA: &str = "6";
    pub const FUKUSHIMA: &str = "7";
    pub const NIIGATA: &str = "15";
    pub const TOYAMA: &str = "16";
    pub const ISHIKAWA: &str = "17";
    pub const FUKUI: &str = "18";
    pub const YAMANASHI: &str = "19";
    pub const NAGANO: &str = "20";
    pub const AICHI: &str = "23";
    pub const GIFU: &str = "21";
    pub const SHIZUOKA: &str = "22";
    pub const MIE: &str = "24";
    pub const OSAKA: &str = "27";
    pub const HYOGO: &str = "28";
    pub const KYOTO: &str = "26";
    pub const SHIGA: &str = "25";
    pub const NARA: &str = "29";
    pub const WAKAYAMA: &str = "30";
    pub const TOTTORI: &str = "31";
    pub const SHIMANE: &str = "32";
    pub const OKAYAMA: &str = "33";
    pub const HIROSHIMA: &str = "34";
    pub const YAMAGUCHI: &str = "35";
    pub const TOKUSHIMA: &str = "36";
    pub const KAGAWA: &str = "37";
    pub const EHIME: &str = "38";
    pub const KOCHI: &str = "39";
    pub const FUKUOKA: &str = "40";
    pub const SAGA: &str = "41";
    pub const NAGASAKI: &str = "42";
    pub const KUMAMOTO: &str = "43";
    pub const OITA: &str = "44";
    pub const MIYAZAKI: &str = "45";
    pub const KAGOSHIMA: &str = "46";
    pub const OKINAWA: &str = "47";
    pub const FULL_REMOTE: &str = "98";
    pub const OVERSEAS: &str = "99";

    pub fn get_area_id(area_name: &str) -> Option<&str> {
        match area_name.to_lowercase().as_str() {
            "tokyo" => Some(TOKYO),
            "kanagawa" => Some(KANAGAWA),
            "chiba" => Some(CHIBA),
            "saitama" => Some(SAITAMA),
            "ibaraki" => Some(IBARAKI),
            "tochigi" => Some(TOCHIGI),
            "gunma" => Some(GUNMA),
            "hokkaido" => Some(HOKKAIDO),
            "aomori" => Some(AOMORI),
            "iwate" => Some(IWATE),
            "miyagi" => Some(MIYAGI),
            "akita" => Some(AKITA),
            "yamagata" => Some(YAMAGATA),
            "fukushima" => Some(FUKUSHIMA),
            "niigata" => Some(NIIGATA),
            "toyama" => Some(TOYAMA),
            "ishikawa" => Some(ISHIKAWA),
            "fukui" => Some(FUKUI),
            "yamanashi" => Some(YAMANASHI),
            "nagano" => Some(NAGANO),
            "aichi" => Some(AICHI),
            "gifu" => Some(GIFU),
            "shizuoka" => Some(SHIZUOKA),
            "mie" => Some(MIE),
            "osaka" => Some(OSAKA),
            "hyogo" => Some(HYOGO),
            "kyoto" => Some(KYOTO),
            "shiga" => Some(SHIGA),
            "nara" => Some(NARA),
            "wakayama" => Some(WAKAYAMA),
            "tottori" => Some(TOTTORI),
            "shimane" => Some(SHIMANE),
            "okayama" => Some(OKAYAMA),
            "hiroshima" => Some(HIROSHIMA),
            "yamaguchi" => Some(YAMAGUCHI),
            "tokushima" => Some(TOKUSHIMA),
            "kagawa" => Some(KAGAWA),
            "ehime" => Some(EHIME),
            "kochi" => Some(KOCHI),
            "fukuoka" => Some(FUKUOKA),
            "saga" => Some(SAGA),
            "nagasaki" => Some(NAGASAKI),
            "kumamoto" => Some(KUMAMOTO),
            "oita" => Some(OITA),
            "miyazaki" => Some(MIYAZAKI),
            "kagoshima" => Some(KAGOSHIMA),
            "okinawa" => Some(OKINAWA),
            "full_remote" => Some(FULL_REMOTE),
            "overseas" => Some(OVERSEAS),
            _ => None,
        }
    }
}

pub mod major_job_type_collection {
    pub const ENGINEER: &str = "190";
    pub const CREATIVE_WEB: &str = "160";
    pub const CREATIVE_GAME: &str = "170";
    pub const PLANNING_MARKETING: &str = "110";
    pub const SALES: &str = "100";
    pub const MANAGEMENT_CXO: &str = "230";
    pub const ACCOUNT_ADMIN_BACK_OFFICE: &str = "120";
    pub const ASSISTANT_OFFICE_WORK: &str = "130";
    pub const SERVICE_PERSONNEL: &str = "140";
    pub const SPECIALIST_FINANCE: &str = "150";
    pub const ENGINEER_ELEC_MECH: &str = "200";
    pub const ARCHITECT_CIVIL_PLANT: &str = "220";

    pub fn get_major_job_id(job_name: &str) -> Option<&str> {
        match job_name.to_lowercase().as_str() {
            "engineer" => Some(ENGINEER),
            "creative_web" => Some(CREATIVE_WEB),
            "creative_game" => Some(CREATIVE_GAME),
            "planning_marketing" => Some(PLANNING_MARKETING),
            "sales" => Some(SALES),
            "management_cxo" => Some(MANAGEMENT_CXO),
            "account_admin_back_office" => Some(ACCOUNT_ADMIN_BACK_OFFICE),
            "assistant_office_work" => Some(ASSISTANT_OFFICE_WORK),
            "service_personnel" => Some(SERVICE_PERSONNEL),
            "specialist_finance" => Some(SPECIALIST_FINANCE),
            "engineer_elec_mech" => Some(ENGINEER_ELEC_MECH),
            "architect_civil_plant" => Some(ARCHITECT_CIVIL_PLANT),
            _ => None,
        }
    }

}

pub mod minor_job_type_collection {
    /* エンジニア職 */
    pub const BACKEND: &str = "190100";
    pub const FRONTEND: &str = "190101";
    pub const SMARTPHONE_APP: &str = "190102";
    pub const SYSTEM_DEV_GENERAL: &str = "190110";
    pub const SYSTEM_DEV_EMBEDDED: &str = "190120";
    pub const INFRA_ENGINEER: &str = "190150";
    pub const NET_MONITOR: &str = "190160";
    pub const NETWORK_ENGINEER: &str = "190151";
    pub const PACKAGE_DEV: &str = "190140";
    pub const PROJECT_MANAGER: &str = "190130";
    pub const CORP_SYS_ENGINEER: &str = "190180";
    pub const PRODUCT_MANAGER: &str = "190200";
    pub const IT_CONSULTANT: &str = "190170";
    pub const DATA_SCIENTIST: &str = "190210";
    pub const QA_ENGINEER: &str = "190220";
    pub const OTHER_SYS: &str = "190190";

    /* クリエイティブWEB */
    pub const WEB_DESIGNER: &str = "160110";
    pub const UI_UX_DESIGNER: &str = "160111";
    pub const WEB_CODER: &str = "160112";
    pub const WEB_PRODUCER: &str = "160100";
    pub const WEB_CONTENT: &str = "160120";
    pub const WEB_WRITER: &str = "160122";

    /* Planning Marketing */
    pub const MARKETING_PR: &str = "110110";
    pub const BUSINESS_PLANNING: &str = "110100";
    pub const WEB_CONSULTANT_SEO_SEM: &str = "110120";
    pub const PRODUCT_PLANNING: &str = "110112";

    /* Sales */
    pub const NEW_BUSINESS_SALES: &str = "100100";
    pub const AGENCY_SALES: &str = "100102";
    pub const PLANNING_SALES: &str = "100104";
    pub const ROUTE_SALES: &str = "100120";
    pub const INSIDE_SALES: &str = "100122";
    pub const SALES_MANAGER: &str = "100160";
    pub const SALES_PLANNING: &str = "100162";
    pub const OVERSEAS_SALES: &str = "100130";
    pub const TECH_SALES: &str = "100140";
    pub const MEDICAL_SALES: &str = "100150";
    pub const CUSTOMER_SUCCESS: &str = "100190";
    pub const OTHER_SALES: &str = "100180";

    /* Account Admin Back Office */
    pub const HR_ADMIN: &str = "120120";
    pub const FINANCE_ACCOUNTING: &str = "120110";
    pub const PUBLIC_RELATIONS: &str = "120100";
    pub const LEGAL_COMPLIANCE: &str = "120130";

    /* Assistant Office Work */
    pub const GENERAL_OFFICE: &str = "130100";
    pub const INTERPRETER_TRANSLATOR: &str = "130102";
    pub const LOGISTICS_TRADE: &str = "130140";
    pub const CUSTOMER_SERVICE: &str = "130130";
    pub const OTHER_OFFICE: &str = "130110";

    /* Service Personnel */
    pub const STORE_MANAGER: &str = "140120";
    pub const SUPERVISOR_AREA_MANAGER: &str = "140112";
    pub const MEDICAL_CARE: &str = "140130";
    pub const CAREER_CONSULTANT: &str = "140100";
    pub const PURCHASING_PROCUREMENT: &str = "140110";
    pub const CALL_CENTER_OPERATOR: &str = "140170";
    pub const OTHER_SERVICE: &str = "140140";

    /* Specialist Finance */
    pub const FINANCE_INSURANCE_FP: &str = "150100";
    pub const REAL_ESTATE_CONSULTANT: &str = "150120";
    pub const STRATEGY_CONSULTANT: &str = "150140";
    pub const ACCOUNTING_CONSULTANT: &str = "150150";
    pub const ORG_HR_CONSULTANT: &str = "150160";
    pub const OTHER_CONSULTANT: &str = "150130";

    /* Engineer Elec Mech */
    pub const PRODUCTION_TECH_PROCESS_DEV: &str = "200150";
    pub const AUTO_CONSTRUCTION_MACHINERY: &str = "200140";
    pub const ANALOG_DIGITAL_SEMI: &str = "200100";
    pub const PRECISION_ELECTRONIC_INSTRUMENT: &str = "200110";
    pub const OTHER_TECH: &str = "200170";

    /* Architect Civil Plant */
    pub const CONSTRUCTION_MANAGEMENT: &str = "220100";
    pub const SURVEY_DESIGN_ESTIMATION: &str = "220110";
    pub const OTHER_ARCH_CIVIL_PLANT: &str = "220120";

    pub fn get_minor_job_id(job_name: &str) -> Option<&str>{
        match job_name.to_lowercase().as_str() {

            /* エンジニア */
            "backend" => Some(BACKEND),
            "frontend" => Some(FRONTEND),
            "smartphone_app" => Some(SMARTPHONE_APP),
            "system_dev_general" => Some(SYSTEM_DEV_GENERAL),
            "system_dev_embedded" => Some(SYSTEM_DEV_EMBEDDED),
            "infra_engineer" => Some(INFRA_ENGINEER),
            "net_monitor" => Some(NET_MONITOR),
            "network_engineer" => Some(NETWORK_ENGINEER),
            "package_dev" => Some(PACKAGE_DEV),
            "project_manager" => Some(PROJECT_MANAGER),
            "corp_sys_engineer" => Some(CORP_SYS_ENGINEER),
            "product_manager" => Some(PRODUCT_MANAGER),
            "it_consultant" => Some(IT_CONSULTANT),
            "data_scientist" => Some(DATA_SCIENTIST),
            "qa_engineer" => Some(QA_ENGINEER),
            "other_sys" => Some(OTHER_SYS),
    
            /* クリエイティブ Web */
            "web_designer" => Some(WEB_DESIGNER),
            "ui_ux_designer" => Some(UI_UX_DESIGNER),
            "web_coder" => Some(WEB_CODER),
            "web_producer" => Some(WEB_PRODUCER),
            "web_content" => Some(WEB_CONTENT),
            "web_writer" => Some(WEB_WRITER),
    
            /* Plannninng Marketing  */
            "marketing_pr" => Some(MARKETING_PR),
            "business_planning" => Some(BUSINESS_PLANNING),
            "web_consultant_seo_sem" => Some(WEB_CONSULTANT_SEO_SEM),
            "product_planning" => Some(PRODUCT_PLANNING),
    
            /* Sales  */
            "new_business_sales" => Some(NEW_BUSINESS_SALES),
            "agency_sales" => Some(AGENCY_SALES),
            "planning_sales" => Some(PLANNING_SALES),
            "route_sales" => Some(ROUTE_SALES),
            "inside_sales" => Some(INSIDE_SALES),
            "sales_manager" => Some(SALES_MANAGER),
            "sales_planning" => Some(SALES_PLANNING),
            "overseas_sales" => Some(OVERSEAS_SALES),
            "tech_sales" => Some(TECH_SALES),
            "medical_sales" => Some(MEDICAL_SALES),
            "customer_success" => Some(CUSTOMER_SUCCESS),
            "other_sales" => Some(OTHER_SALES),
    
                    /* Account Admin Back Office */
            "hr_admin" => Some(HR_ADMIN),
            "finance_accounting" => Some(FINANCE_ACCOUNTING),
            "public_relations" => Some(PUBLIC_RELATIONS),
            "legal_compliance" => Some(LEGAL_COMPLIANCE),
    
            /* Assistant Office Work */
            "general_office" => Some(GENERAL_OFFICE),
            "interpreter_translator" => Some(INTERPRETER_TRANSLATOR),
            "logistics_trade" => Some(LOGISTICS_TRADE),
            "customer_service" => Some(CUSTOMER_SERVICE),
            "other_office" => Some(OTHER_OFFICE),
    
            /* Service Personnel */
            "store_manager" => Some(STORE_MANAGER),
            "supervisor_area_manager" => Some(SUPERVISOR_AREA_MANAGER),
            "medical_care" => Some(MEDICAL_CARE),
            "career_consultant" => Some(CAREER_CONSULTANT),
            "purchasing_procurement" => Some(PURCHASING_PROCUREMENT),
            "call_center_operator" => Some(CALL_CENTER_OPERATOR),
            "other_service" => Some(OTHER_SERVICE),
    
            /* Specialist Finance */
            "finance_insurance_fp" => Some(FINANCE_INSURANCE_FP),
            "real_estate_consultant" => Some(REAL_ESTATE_CONSULTANT),
            "strategy_consultant" => Some(STRATEGY_CONSULTANT),
            "accounting_consultant" => Some(ACCOUNTING_CONSULTANT),
            "org_hr_consultant" => Some(ORG_HR_CONSULTANT),
            "other_consultant" => Some(OTHER_CONSULTANT),
    
            /* Engineer Elec Mech */
            "production_tech_process_dev" => Some(PRODUCTION_TECH_PROCESS_DEV),
            "auto_construction_machinery" => Some(AUTO_CONSTRUCTION_MACHINERY),
            "analog_digital_semi" => Some(ANALOG_DIGITAL_SEMI),
            "precision_electronic_instrument" => Some(PRECISION_ELECTRONIC_INSTRUMENT),
            "other_tech" => Some(OTHER_TECH),
    
            /* Architect Civil Plant */
            "construction_management" => Some(CONSTRUCTION_MANAGEMENT),
            "survey_design_estimation" => Some(SURVEY_DESIGN_ESTIMATION),
            "other_arch_civil_plant" => Some(OTHER_ARCH_CIVIL_PLANT),
            
            /* 何もない */
            _ => None,
        }    
    }
}
